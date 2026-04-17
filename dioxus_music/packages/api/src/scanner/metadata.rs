use lofty::{
    file::TaggedFileExt,
    probe::Probe,
    tag::{Accessor, ItemKey},
};
use std::path::Path;

/// All metadata extracted from a single audio file.
#[derive(Debug, Clone)]
pub struct TrackMetadata {
    pub title: String,
    pub artist: String,       // track artist
    pub album_artist: String, // falls back to artist if missing
    pub album: String,
    pub genre: String,
    pub year: Option<i32>,
    pub track_number: Option<i32>,
    pub disc_number: i32,
    pub duration_ticks: i64,   // 100-nanosecond ticks
    pub container: String,     // mp3 | flac | ogg | opus
    pub bit_rate: Option<i32>, // kbps
    pub sample_rate: Option<i32>,
    pub channels: Option<i32>,
    pub has_embedded_art: bool,
}

/// Extracts metadata from a music file. Returns None if the file cannot be read.
pub fn read_metadata(path: &Path) -> Option<TrackMetadata> {
    use lofty::file::AudioFile;

    let tagged_file = Probe::open(path).ok()?.read().ok()?;

    let props = tagged_file.properties();
    let duration_ticks = props.duration().as_millis() as i64 * 10_000;
    let bit_rate = props.audio_bitrate().map(|b| b as i32);
    let sample_rate = props.sample_rate().map(|r| r as i32);
    let channels = props.channels().map(|c| c as i32);

    let container = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .unwrap_or_else(|| "unknown".to_string());

    let tag = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag());
    // pictures() returns &[Picture], use .is_empty() to check
    let has_embedded_art = tag.map(|t| !t.pictures().is_empty()).unwrap_or(false);

    let title = tag
        .and_then(|t| t.title().map(|s| s.into_owned()))
        .or_else(|| path.file_stem().and_then(|s| s.to_str()).map(String::from))
        .unwrap_or_default();

    let artist = tag
        .and_then(|t| t.artist().map(|s| s.into_owned()))
        .unwrap_or_default();

    let album_artist = tag
        .and_then(|t| {
            // get_string takes ItemKey by value
            t.get_string(ItemKey::AlbumArtist).map(String::from)
        })
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| artist.clone());

    let album = tag
        .and_then(|t| t.album().map(|s| s.into_owned()))
        .unwrap_or_default();

    let genre = tag
        .and_then(|t| t.genre().map(|s| s.into_owned()))
        .unwrap_or_default();

    let year = tag
        .and_then(|t| t.date())
        .and_then(|d| i32::try_from(d.year).ok());

    let track_number = tag
        .and_then(|t| t.track())
        .and_then(|n| i32::try_from(n).ok());

    let disc_number = tag
        .and_then(|t| t.disk())
        .and_then(|n| i32::try_from(n).ok())
        .unwrap_or(1);

    Some(TrackMetadata {
        title,
        artist,
        album_artist,
        album,
        genre,
        year,
        track_number,
        disc_number,
        duration_ticks,
        container,
        bit_rate,
        sample_rate,
        channels,
        has_embedded_art,
    })
}

/// Extract the raw bytes of the primary embedded cover art, if any.
pub fn read_embedded_art(path: &Path) -> Option<(Vec<u8>, String)> {
    let tagged_file = Probe::open(path).ok()?.read().ok()?;

    let tag = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag())?;
    // pictures() returns &[Picture]; use .first() to get the first element
    let pic = tag.pictures().first()?;
    // mime_type() returns Option<&MimeType>; MimeType implements Display via .to_string()
    let mime = pic
        .mime_type()
        .map(|m| m.to_string())
        .unwrap_or_else(|| "image/jpeg".to_string());
    Some((pic.data().to_vec(), mime))
}

/// Compute the sort name for an artist:
/// "The Beatles" → "Beatles, The", "A Tribe Called Quest" → "Tribe Called Quest, A"
pub fn make_sort_name(name: &str) -> String {
    for article in ["The ", "the ", "A ", "a ", "An ", "an "] {
        if let Some(rest) = name.strip_prefix(article) {
            return format!("{}, {}", rest, article.trim());
        }
    }
    name.to_string()
}

/// Normalize a string for deduplication: lowercase + trim.
pub fn normalize(s: &str) -> String {
    s.trim().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_name_strips_the() {
        assert_eq!(make_sort_name("The National"), "National, The");
    }

    #[test]
    fn sort_name_strips_a() {
        assert_eq!(
            make_sort_name("A Tribe Called Quest"),
            "Tribe Called Quest, A"
        );
    }

    #[test]
    fn sort_name_strips_an() {
        assert_eq!(make_sort_name("An Horse"), "Horse, An");
    }

    #[test]
    fn sort_name_no_article() {
        assert_eq!(make_sort_name("Radiohead"), "Radiohead");
    }

    #[test]
    fn normalize_trims_and_lowercases() {
        assert_eq!(normalize("  The Beatles  "), "the beatles");
    }
}
