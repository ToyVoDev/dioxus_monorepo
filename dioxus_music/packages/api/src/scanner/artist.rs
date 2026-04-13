use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    db::{
        models::{Artist, NewArtist},
        schema::artists,
    },
    scanner::metadata::{make_sort_name, normalize},
};

/// Find an existing artist by normalized name, or insert a new one.
/// Returns the artist UUID.
pub async fn find_or_create(
    conn: &mut AsyncPgConnection,
    name: &str,
) -> Result<Uuid, diesel::result::Error> {
    let norm = normalize(name);

    // Try to find existing artist by normalized name (case-insensitive, parameterized).
    let existing: Option<Artist> = artists::table
        .filter(
            diesel::dsl::sql::<diesel::sql_types::Bool>("LOWER(TRIM(name)) = ")
                .bind::<diesel::sql_types::Text, _>(&norm),
        )
        .first(conn)
        .await
        .optional()?;

    if let Some(a) = existing {
        return Ok(a.id);
    }

    // Insert new artist.
    let display_name = if name.trim().is_empty() {
        "Unknown Artist".to_string()
    } else {
        name.trim().to_string()
    };

    let new_artist = NewArtist {
        id: Uuid::new_v4(),
        name: display_name.clone(),
        sort_name: make_sort_name(&display_name),
    };

    let inserted: Artist = diesel::insert_into(artists::table)
        .values(&new_artist)
        .get_result(conn)
        .await?;

    Ok(inserted.id)
}
