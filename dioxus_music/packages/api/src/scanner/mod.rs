pub mod album;
pub mod artist;
pub mod genres;
pub mod images;
pub mod metadata;

use std::path::Path;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;
use walkdir::WalkDir;

use crate::{
    db::{
        models::{NewImage, NewTrack},
        schema::{albums, artists, genres as genres_table, images as images_table, playlist_items, playlists, tracks, user_data},
    },
    state::AppState,
};

/// Incremental scan: processes only new files, removes deleted ones.
pub async fn quick_scan(state: AppState) {
    tracing::info!("Starting quick scan of {:?}", state.music_dir);
    let paths = collect_music_paths(&state.music_dir);

    let Ok(mut conn) = state.pool.get().await else {
        tracing::error!("quick_scan: failed to get DB connection");
        return;
    };

    // Get existing file paths from DB.
    let existing_paths: Vec<String> = tracks::table
        .select(tracks::file_path)
        .load(&mut conn)
        .await
        .unwrap_or_default();
    let existing_set: std::collections::HashSet<String> =
        existing_paths.into_iter().collect();

    // New files not yet in DB.
    let new_paths: Vec<_> = paths
        .iter()
        .filter(|p| !existing_set.contains(p.to_str().unwrap_or("")))
        .cloned()
        .collect();

    // Files in DB that no longer exist on disk.
    let disk_set: std::collections::HashSet<String> = paths
        .iter()
        .filter_map(|p| p.to_str().map(String::from))
        .collect();
    let stale: Vec<String> = existing_set
        .difference(&disk_set)
        .cloned()
        .collect();

    if !stale.is_empty() {
        tracing::info!("Removing {} stale tracks", stale.len());
        diesel::delete(tracks::table.filter(tracks::file_path.eq_any(&stale)))
            .execute(&mut conn)
            .await
            .ok();
    }

    if new_paths.is_empty() {
        tracing::info!("Quick scan: no new files");
        genres::refresh(&mut conn).await.ok();
        return;
    }

    tracing::info!("Quick scan: processing {} new files", new_paths.len());
    ingest_files(&state, &new_paths).await;

    let Ok(mut conn) = state.pool.get().await else { return };
    genres::refresh(&mut conn).await.ok();
    tracing::info!("Quick scan complete");
}

/// Full rescan: truncates all library tables, clears image cache, re-scans.
pub async fn full_scan(state: AppState) {
    tracing::info!("Starting full scan of {:?}", state.music_dir);

    let Ok(mut conn) = state.pool.get().await else {
        tracing::error!("full_scan: failed to get DB connection");
        return;
    };

    // Clear image cache directory.
    if state.image_cache_dir.exists() {
        std::fs::remove_dir_all(&state.image_cache_dir).ok();
    }
    if let Err(e) = std::fs::create_dir_all(&state.image_cache_dir) {
        tracing::error!("full_scan: failed to create image cache dir {:?}: {e}", state.image_cache_dir);
        return;
    }

    // Truncate tables in dependency order.
    diesel::delete(user_data::table).execute(&mut conn).await.ok();
    diesel::delete(playlist_items::table).execute(&mut conn).await.ok();
    diesel::delete(playlists::table).execute(&mut conn).await.ok();
    diesel::delete(images_table::table).execute(&mut conn).await.ok();
    diesel::delete(genres_table::table).execute(&mut conn).await.ok();
    diesel::delete(tracks::table).execute(&mut conn).await.ok();
    diesel::delete(albums::table).execute(&mut conn).await.ok();
    diesel::delete(artists::table).execute(&mut conn).await.ok();

    drop(conn);

    let paths = collect_music_paths(&state.music_dir);
    tracing::info!("Full scan: processing {} files", paths.len());
    ingest_files(&state, &paths).await;

    let Ok(mut conn) = state.pool.get().await else { return };
    genres::refresh(&mut conn).await.ok();
    tracing::info!("Full scan complete");
}

/// Process a batch of file paths: extract metadata, resolve artist/album, insert track.
async fn ingest_files(state: &AppState, paths: &[std::path::PathBuf]) {
    // Group paths by directory for album-level image extraction.
    let mut by_dir: std::collections::HashMap<std::path::PathBuf, Vec<std::path::PathBuf>> =
        std::collections::HashMap::new();
    for path in paths {
        if let Some(dir) = path.parent() {
            by_dir.entry(dir.to_path_buf()).or_default().push(path.clone());
        }
    }

    for (dir, dir_paths) in &by_dir {
        process_directory(state, dir, dir_paths).await;
    }
}

/// Process all files in one directory (one album's worth of tracks).
async fn process_directory(
    state: &AppState,
    dir: &Path,
    paths: &[std::path::PathBuf],
) {
    // Track cover extraction state to avoid redundant DB queries after first attempt.
    let mut cover_extracted = false;
    // Track last resolved album for post-loop artist image propagation.
    let mut last_album_id: Option<Uuid> = None;

    for path in paths {
        // Read metadata on a blocking thread to avoid blocking the async runtime.
        let path_clone = path.clone();
        let meta = tokio::task::spawn_blocking(move || metadata::read_metadata(&path_clone))
            .await
            .ok()
            .flatten();

        let Some(meta) = meta else {
            tracing::warn!("Could not read metadata for {:?}", path);
            continue;
        };

        let Ok(mut conn) = state.pool.get().await else { continue };

        // Resolve track artist — skip track if DB is unreachable.
        let track_artist_id = match artist::find_or_create(&mut conn, &meta.artist).await {
            Ok(id) => id,
            Err(e) => {
                tracing::error!("Failed to resolve artist for {:?}: {e}", path);
                continue;
            }
        };

        let album_artist_id = if meta.album_artist != meta.artist {
            artist::find_or_create(&mut conn, &meta.album_artist)
                .await
                .unwrap_or(track_artist_id)
        } else {
            track_artist_id
        };

        // Resolve album per-track so compilation folders assign each track to its correct album.
        let the_album_id = match album::find_or_create(&mut conn, &meta.album, album_artist_id, meta.year).await {
            Ok(id) => id,
            Err(e) => {
                tracing::error!("Failed to resolve album for {:?}: {e}", path);
                continue;
            }
        };
        last_album_id = Some(the_album_id);

        let new_track = NewTrack {
            id: Uuid::new_v4(),
            title: meta.title.clone(),
            sort_title: meta.title.to_ascii_lowercase(),
            artist_id: track_artist_id,
            album_id: Some(the_album_id),
            genre: meta.genre.clone(),
            duration_ticks: meta.duration_ticks,
            track_number: meta.track_number,
            disc_number: meta.disc_number,
            file_path: path.to_str().unwrap_or("").to_string(),
            container: meta.container.clone(),
            bit_rate: meta.bit_rate,
            sample_rate: meta.sample_rate,
            channels: meta.channels,
        };

        if let Err(e) = diesel::insert_into(crate::db::schema::tracks::table)
            .values(&new_track)
            .on_conflict(crate::db::schema::tracks::file_path)
            .do_nothing()
            .execute(&mut conn)
            .await
        {
            tracing::error!("Failed to insert track {:?}: {e}", path);
        }

        // Attempt cover art extraction once per directory (or until we get art).
        // try_extract_album_cover is idempotent, so later tracks won't double-write,
        // but we skip extra DB round-trips once we know we've already tried.
        if !cover_extracted {
            try_extract_album_cover(state, dir, path, the_album_id, &meta).await;
            cover_extracted = true;
        }
    }

    // Propagate the album's Primary image to its artist if the artist has none.
    if let Some(album_id) = last_album_id {
        if let Ok(mut conn) = state.pool.get().await {
            propagate_artist_image_from_album(&mut conn, album_id).await;
        }
    }
}

/// Extract and cache cover art for an album. Tries embedded art first, then folder cover.
async fn try_extract_album_cover(
    state: &AppState,
    dir: &Path,
    first_track: &Path,
    album_id: Uuid,
    meta: &metadata::TrackMetadata,
) {
    let Ok(mut conn) = state.pool.get().await else { return };

    // Skip if album already has an image.
    let existing: Option<crate::db::models::Image> =
        images_table::table
            .filter(images_table::item_id.eq(album_id))
            .filter(images_table::image_type.eq("Primary"))
            .first(&mut conn)
            .await
            .optional()
            .unwrap_or(None);

    if existing.is_some() {
        return;
    }

    // Try embedded art.
    let art_data: Option<(Vec<u8>, String)> = if meta.has_embedded_art {
        let path = first_track.to_path_buf();
        tokio::task::spawn_blocking(move || metadata::read_embedded_art(&path))
            .await
            .ok()
            .flatten()
    } else {
        None
    };

    // Fallback to folder cover.
    let art_data = if art_data.is_none() {
        let dir = dir.to_path_buf();
        tokio::task::spawn_blocking(move || {
            images::find_folder_cover(&dir).and_then(|p| {
                let data = std::fs::read(&p).ok()?;
                let ext = p.extension()?.to_str()?.to_ascii_lowercase();
                let mime = match ext.as_str() {
                    "png" => "image/png".to_string(),
                    "webp" => "image/webp".to_string(),
                    _ => "image/jpeg".to_string(),
                };
                Some((data, mime))
            })
        })
        .await
        .ok()
        .flatten()
    } else {
        art_data
    };

    let Some((data, mime)) = art_data else { return };

    let ext = images::mime_to_ext(&mime);
    let tag = images::compute_tag(&data);
    let cache_dir = state.image_cache_dir.clone();

    let dest = match tokio::task::spawn_blocking(move || {
        images::write_image_cache(&cache_dir, album_id, "Primary", &data, ext)
    })
    .await
    .ok()
    .and_then(|r| r.ok())
    {
        Some(p) => p,
        None => {
            tracing::warn!("Failed to write image cache for album {album_id}");
            return;
        }
    };

    let (width, height) = {
        let dest_clone = dest.clone();
        tokio::task::spawn_blocking(move || images::read_dimensions(&dest_clone))
            .await
            .ok()
            .flatten()
            .map(|(w, h)| (Some(w), Some(h)))
            .unwrap_or((None, None))
    };

    let new_image = NewImage {
        item_id: album_id,
        image_type: "Primary".to_string(),
        file_path: dest.to_str().unwrap_or("").to_string(),
        tag,
        width,
        height,
    };

    diesel::insert_into(images_table::table)
        .values(&new_image)
        .on_conflict((
            images_table::item_id,
            images_table::image_type,
        ))
        .do_nothing()
        .execute(&mut conn)
        .await
        .ok();
}

/// Give the album artist the same Primary image as one of their albums, if they don't have one.
async fn propagate_artist_image_from_album(
    conn: &mut diesel_async::AsyncPgConnection,
    album_id: Uuid,
) {
    let album: Option<crate::db::models::Album> = albums::table
        .filter(albums::id.eq(album_id))
        .first(conn)
        .await
        .optional()
        .unwrap_or(None);

    let Some(album) = album else { return };
    let artist_id = album.artist_id;

    // Check if artist already has an image.
    let has_image: bool = images_table::table
        .filter(images_table::item_id.eq(artist_id))
        .filter(images_table::image_type.eq("Primary"))
        .count()
        .get_result::<i64>(conn)
        .await
        .unwrap_or(0)
        > 0;

    if has_image {
        return;
    }

    // Copy album's image reference to artist.
    let album_image: Option<crate::db::models::Image> = images_table::table
        .filter(images_table::item_id.eq(album_id))
        .filter(images_table::image_type.eq("Primary"))
        .first(conn)
        .await
        .optional()
        .unwrap_or(None);

    if let Some(img) = album_image {
        let new_image = NewImage {
            item_id: artist_id,
            image_type: "Primary".to_string(),
            file_path: img.file_path,
            tag: img.tag,
            width: img.width,
            height: img.height,
        };
        diesel::insert_into(images_table::table)
            .values(&new_image)
            .on_conflict((images_table::item_id, images_table::image_type))
            .do_nothing()
            .execute(conn)
            .await
            .ok();
    }
}

fn collect_music_paths(music_dir: &Path) -> Vec<std::path::PathBuf> {
    WalkDir::new(music_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let path = e.into_path();
            let ext = path.extension()?.to_ascii_lowercase();
            let ext = ext.to_str()?;
            matches!(ext, "flac" | "mp3" | "ogg" | "opus").then_some(path)
        })
        .collect()
}
