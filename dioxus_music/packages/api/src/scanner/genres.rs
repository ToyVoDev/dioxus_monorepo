use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::db::{models::NewGenre, schema::{genres, tracks}};

/// UUIDv5 namespace for deterministic genre UUIDs.
/// Using the DNS namespace UUID as a stable base.
const NAMESPACE: uuid::Uuid = uuid::Uuid::from_bytes([
    0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1,
    0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8,
]);

/// Derive a deterministic UUID for a genre name.
pub fn genre_uuid(name: &str) -> Uuid {
    Uuid::new_v5(&NAMESPACE, name.trim().to_lowercase().as_bytes())
}

/// Repopulate the `genres` table from distinct non-empty genres in `tracks`.
/// UUIDs are deterministic so existing `user_data` references survive.
pub async fn refresh(conn: &mut AsyncPgConnection) -> Result<(), diesel::result::Error> {
    // Get distinct genres from tracks.
    let distinct: Vec<String> = tracks::table
        .select(tracks::genre)
        .filter(tracks::genre.ne(""))
        .distinct()
        .load(conn)
        .await?;

    // Delete existing genres not in new set.
    let new_names: Vec<String> = distinct.clone();
    diesel::delete(genres::table.filter(genres::name.ne_all(new_names)))
        .execute(conn)
        .await?;

    // Upsert each genre.
    for name in distinct {
        let new_genre = NewGenre {
            id: genre_uuid(&name),
            name: name.clone(),
        };
        diesel::insert_into(genres::table)
            .values(&new_genre)
            .on_conflict(genres::id)
            .do_nothing()
            .execute(conn)
            .await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genre_uuid_is_deterministic() {
        assert_eq!(genre_uuid("Jazz"), genre_uuid("Jazz"));
    }

    #[test]
    fn genre_uuid_is_case_insensitive() {
        assert_eq!(genre_uuid("Jazz"), genre_uuid("jazz"));
        assert_eq!(genre_uuid("Jazz"), genre_uuid("  Jazz  "));
    }

    #[test]
    fn genre_uuid_differs_for_different_names() {
        assert_ne!(genre_uuid("Jazz"), genre_uuid("Blues"));
    }
}
