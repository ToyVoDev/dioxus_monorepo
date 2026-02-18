use crate::db::DbPool;
use crate::schema::tracks;
use axum::{
    Extension,
    body::Body,
    extract::Path,
    http::{StatusCode, header},
    response::Response,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::path::PathBuf;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

pub async fn stream_track(
    Extension(pool): Extension<DbPool>,
    Path(track_id): Path<Uuid>,
) -> Result<Response, (StatusCode, &'static str)> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB pool error"))?;

    let file_path: String = tracks::table
        .filter(tracks::id.eq(track_id))
        .select(tracks::file_path)
        .first(&mut conn)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "Track not found"))?;

    let path = PathBuf::from(&file_path);

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let content_type = match ext.as_str() {
        "flac" => "audio/flac",
        "mp3" => "audio/mpeg",
        "ogg" | "opus" => "audio/ogg",
        _ => "application/octet-stream",
    };

    let file = tokio::fs::File::open(&path)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to open file"))?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, content_type)
        .body(body)
        .unwrap())
}
