use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    db::{
        models::{Album, NewAlbum},
        schema::albums,
    },
    scanner::metadata::normalize,
};

/// Find an existing album by normalized (title, artist_id), or insert a new one.
/// Returns the album UUID.
pub async fn find_or_create(
    conn: &mut AsyncPgConnection,
    title: &str,
    artist_id: Uuid,
    year: Option<i32>,
) -> Result<Uuid, diesel::result::Error> {
    let norm_title = normalize(title);

    // Parameterized comparison — avoids SQL injection via music file tags.
    let existing: Option<Album> = albums::table
        .filter(albums::artist_id.eq(artist_id))
        .filter(
            diesel::dsl::sql::<diesel::sql_types::Bool>("LOWER(TRIM(title)) = ")
                .bind::<diesel::sql_types::Text, _>(&norm_title),
        )
        .first(conn)
        .await
        .optional()?;

    if let Some(a) = existing {
        return Ok(a.id);
    }

    let display_title = if title.trim().is_empty() {
        "Unknown Album".to_string()
    } else {
        title.trim().to_string()
    };

    let new_album = NewAlbum {
        id: Uuid::new_v4(),
        title: display_title.clone(),
        sort_title: display_title.to_ascii_lowercase(),
        artist_id,
        year,
    };

    let inserted: Album = diesel::insert_into(albums::table)
        .values(&new_album)
        .get_result(conn)
        .await?;

    Ok(inserted.id)
}
