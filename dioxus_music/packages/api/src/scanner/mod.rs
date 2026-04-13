mod album;
mod artist;
mod genres;
mod images;
mod metadata;

use crate::state::AppState;

/// Incremental scan: skips files already in `tracks`, handles deletions, fast.
/// Run at startup in a background task.
pub async fn quick_scan(state: AppState) {
    // implemented in Task 7
    let _ = state;
}

/// Full rescan: truncates tracks/albums/artists/genres/images tables and image
/// cache, then re-scans everything. Triggered by POST /custom/library/rescan.
pub async fn full_scan(state: AppState) {
    // implemented in Task 7
    let _ = state;
}

/// Collect all music file paths under the music directory.
fn collect_music_paths(music_dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    use walkdir::WalkDir;
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
