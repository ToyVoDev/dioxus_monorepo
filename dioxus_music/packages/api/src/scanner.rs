use crate::db::DbPool;
use crate::models::NewTrack;
use crate::schema::tracks;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use lofty::file::AudioFile;
use lofty::file::TaggedFileExt;
use lofty::tag::Accessor;
use std::collections::HashSet;
use walkdir::WalkDir;

const SUPPORTED_EXTENSIONS: &[&str] = &["flac", "opus", "ogg", "mp3"];

/// Quick startup scan: only reads metadata for files not already in the DB.
/// Removes DB rows for files no longer on disk.
pub async fn quick_scan(pool: DbPool) {
    // Walk filesystem to discover all music file paths
    let disk_paths: Vec<String> = tokio::task::spawn_blocking(collect_music_paths)
        .await
        .expect("Scanner thread panicked");

    if disk_paths.is_empty() {
        tracing::info!("No music files found on disk");
        return;
    }

    let mut conn = pool.get().await.expect("Failed to get DB connection");

    // Query existing file_paths from DB
    let existing_paths: Vec<String> = tracks::table
        .select(tracks::file_path)
        .load(&mut conn)
        .await
        .expect("Failed to query existing tracks");

    let existing_set: HashSet<&str> = existing_paths.iter().map(|s| s.as_str()).collect();

    // Find new files that aren't in the DB yet
    let new_paths: Vec<String> = disk_paths
        .iter()
        .filter(|p| !existing_set.contains(p.as_str()))
        .cloned()
        .collect();
    let new_count = new_paths.len();

    // Only read metadata for genuinely new files
    if !new_paths.is_empty() {
        let new_tracks = tokio::task::spawn_blocking(move || read_metadata(&new_paths))
            .await
            .expect("Metadata reader thread panicked");

        for chunk in new_tracks.chunks(100) {
            diesel::insert_into(tracks::table)
                .values(chunk)
                .on_conflict(tracks::file_path)
                .do_nothing()
                .execute(&mut conn)
                .await
                .expect("Failed to insert new tracks");
        }

        tracing::info!("Quick scan: inserted {} new tracks", new_tracks.len());
    }

    // Delete rows for files no longer on disk
    let disk_set: HashSet<&str> = disk_paths.iter().map(|s| s.as_str()).collect();
    let stale_paths: Vec<&str> = existing_paths
        .iter()
        .filter(|p| !disk_set.contains(p.as_str()))
        .map(|s| s.as_str())
        .collect();

    if !stale_paths.is_empty() {
        diesel::delete(tracks::table.filter(tracks::file_path.eq_any(&stale_paths)))
            .execute(&mut conn)
            .await
            .expect("Failed to prune stale tracks");

        tracing::info!("Quick scan: removed {} stale tracks", stale_paths.len());
    }

    tracing::info!(
        "Quick scan complete: {} tracks on disk, {} already existed",
        disk_paths.len(),
        disk_paths.len() - new_count
    );
}

/// Full rescan: deletes all existing tracks and re-reads every file from scratch.
pub async fn full_scan(pool: DbPool) {
    let mut conn = pool.get().await.expect("Failed to get DB connection");

    // Wipe existing data
    diesel::delete(tracks::table)
        .execute(&mut conn)
        .await
        .expect("Failed to truncate tracks table");

    // Walk and read metadata for all files
    let new_tracks = tokio::task::spawn_blocking(scan_music_dir)
        .await
        .expect("Scanner thread panicked");

    if new_tracks.is_empty() {
        tracing::info!("Full rescan: no music files found");
        return;
    }

    let count = new_tracks.len();
    for chunk in new_tracks.chunks(100) {
        diesel::insert_into(tracks::table)
            .values(chunk)
            .execute(&mut conn)
            .await
            .expect("Failed to insert tracks");
    }

    tracing::info!("Full rescan complete: {} tracks indexed", count);
}

/// Collect all music file paths without reading metadata.
fn collect_music_paths() -> Vec<String> {
    let music_dir = dirs::audio_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("Music")))
        .expect("Could not determine music directory");

    if !music_dir.exists() {
        tracing::warn!("Music directory does not exist: {}", music_dir.display());
        return Vec::new();
    }

    let mut paths = Vec::new();
    for entry in WalkDir::new(&music_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(e) => e.to_lowercase(),
            None => continue,
        };

        if SUPPORTED_EXTENSIONS.contains(&ext.as_str()) {
            paths.push(path.to_string_lossy().to_string());
        }
    }

    paths
}

/// Read metadata from a set of file paths, returning NewTrack for each.
fn read_metadata(paths: &[String]) -> Vec<NewTrack> {
    let mut tracks = Vec::with_capacity(paths.len());

    for file_path in paths {
        let path = std::path::Path::new(file_path);
        let tagged = match lofty::read_from_path(path) {
            Ok(t) => t,
            Err(e) => {
                tracing::warn!("Failed to read tags from {}: {}", path.display(), e);
                continue;
            }
        };

        let tag = tagged.primary_tag().or_else(|| tagged.first_tag());

        let title = tag
            .and_then(|t| t.title().map(|s| s.to_string()))
            .unwrap_or_else(|| {
                path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            });
        let artist = tag
            .and_then(|t| t.artist().map(|s| s.to_string()))
            .unwrap_or_default();
        let album = tag
            .and_then(|t| t.album().map(|s| s.to_string()))
            .unwrap_or_default();
        let genre = tag
            .and_then(|t| t.genre().map(|s| s.to_string()))
            .unwrap_or_default();

        let duration_secs = tagged.properties().duration().as_secs() as i32;

        tracks.push(NewTrack {
            title,
            artist,
            album,
            genre,
            duration_secs,
            file_path: file_path.clone(),
        });
    }

    tracks
}

/// Walk the full music directory and read metadata for every file.
fn scan_music_dir() -> Vec<NewTrack> {
    let paths = collect_music_paths();
    tracing::info!("Found {} music files, reading metadata...", paths.len());
    read_metadata(&paths)
}
