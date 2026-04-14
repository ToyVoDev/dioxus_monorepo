use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Cached image info ready to insert into the `images` table.
#[derive(Debug)]
pub struct CachedImage {
    pub item_id: Uuid,
    pub image_type: String, // "Primary"
    pub file_path: String,  // absolute path in cache dir
    pub tag: String,        // SHA-256 hex
    pub width: Option<i32>,
    pub height: Option<i32>,
}

/// Compute SHA-256 hex tag for image bytes.
pub fn compute_tag(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Determine the file extension for an image given its MIME type.
pub fn mime_to_ext(mime: &str) -> &'static str {
    match mime {
        "image/png" => "png",
        "image/webp" => "webp",
        _ => "jpg",
    }
}

/// Find a cover image file in a directory (cover.jpg, cover.jpeg, cover.png, cover.webp,
/// folder.jpg, folder.png — all case-insensitive).
pub fn find_folder_cover(dir: &Path) -> Option<PathBuf> {
    let stems = ["cover", "folder", "album", "front"];
    let exts = ["jpg", "jpeg", "png", "webp"];

    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_ascii_lowercase())
            .unwrap_or_default();
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_ascii_lowercase())
            .unwrap_or_default();

        if stems.contains(&stem.as_str()) && exts.contains(&ext.as_str()) {
            return Some(path);
        }
    }
    None
}

/// Write image bytes to `{cache_dir}/{item_id}/Primary.{ext}`.
/// Returns the destination path.
pub fn write_image_cache(
    cache_dir: &Path,
    item_id: Uuid,
    image_type: &str,
    data: &[u8],
    ext: &str,
) -> std::io::Result<PathBuf> {
    let item_dir = cache_dir.join(item_id.to_string());
    std::fs::create_dir_all(&item_dir)?;
    let dest = item_dir.join(format!("{image_type}.{ext}"));
    std::fs::write(&dest, data)?;
    Ok(dest)
}

/// Read image dimensions using the `image` crate.
/// Returns None if the file cannot be decoded.
pub fn read_dimensions(path: &Path) -> Option<(i32, i32)> {
    let reader = image::ImageReader::open(path).ok()?;
    let (w, h) = reader.into_dimensions().ok()?;
    Some((w as i32, h as i32))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_temp_dir() -> TempDir {
        tempfile::TempDir::new().unwrap()
    }

    #[test]
    fn compute_tag_is_deterministic() {
        let data = b"hello";
        assert_eq!(compute_tag(data), compute_tag(data));
    }

    #[test]
    fn compute_tag_differs_for_different_data() {
        assert_ne!(compute_tag(b"a"), compute_tag(b"b"));
    }

    #[test]
    fn mime_to_ext_png() {
        assert_eq!(mime_to_ext("image/png"), "png");
    }

    #[test]
    fn mime_to_ext_defaults_to_jpg() {
        assert_eq!(mime_to_ext("image/jpeg"), "jpg");
        assert_eq!(mime_to_ext("image/unknown"), "jpg");
    }

    #[test]
    fn find_folder_cover_finds_cover_jpg() {
        let dir = make_temp_dir();
        let cover = dir.path().join("cover.jpg");
        std::fs::write(&cover, b"fake image").unwrap();
        assert_eq!(find_folder_cover(dir.path()), Some(cover));
    }

    #[test]
    fn find_folder_cover_finds_folder_png() {
        let dir = make_temp_dir();
        let cover = dir.path().join("folder.png");
        std::fs::write(&cover, b"fake image").unwrap();
        assert_eq!(find_folder_cover(dir.path()), Some(cover));
    }

    #[test]
    fn find_folder_cover_case_insensitive() {
        let dir = make_temp_dir();
        let cover = dir.path().join("Cover.JPG");
        std::fs::write(&cover, b"fake image").unwrap();
        assert_eq!(find_folder_cover(dir.path()), Some(cover));
    }

    #[test]
    fn find_folder_cover_returns_none_when_absent() {
        let dir = make_temp_dir();
        assert_eq!(find_folder_cover(dir.path()), None);
    }

    #[test]
    fn write_image_cache_creates_file() {
        let dir = make_temp_dir();
        let id = Uuid::new_v4();
        let path = write_image_cache(dir.path(), id, "Primary", b"imgdata", "jpg").unwrap();
        assert!(path.exists());
        assert_eq!(std::fs::read(&path).unwrap(), b"imgdata");
    }
}
