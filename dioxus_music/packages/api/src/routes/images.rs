use axum::{
    Router,
    body::Body,
    extract::{Path, Query, State},
    http::{StatusCode, header},
    response::Response,
    routing::get,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    db::{models::Image, schema::{artists, images, tracks}},
    error::ApiError,
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/Items/{item_id}/Images", get(list_item_images))
        .route("/Items/{item_id}/Images/{image_type}", get(get_item_image))
        .route("/Artists/{name}/Images/{image_type}/{index}", get(get_artist_image))
        .route("/MusicGenres/{name}/Images/{image_type}/{index}", get(get_genre_image))
}

#[derive(Debug, Deserialize)]
pub struct ImageQuery {
    pub tag: Option<String>,
    #[serde(rename = "maxWidth")]
    pub max_width: Option<u32>,
    #[serde(rename = "maxHeight")]
    pub max_height: Option<u32>,
}

async fn serve_image(path: &str) -> Result<Response, ApiError> {
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg");
    let mime = match ext {
        "png" => "image/png",
        "webp" => "image/webp",
        _ => "image/jpeg",
    };

    let data = tokio::fs::read(path)
        .await
        .map_err(|_| ApiError::NotFound("Image file not found on disk".to_string()))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime)
        .header(header::CACHE_CONTROL, "public, max-age=31536000, immutable")
        .body(Body::from(data))
        .unwrap())
}

/// GET /Items/{itemId}/Images — list available image types for an item
async fn list_item_images(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(item_id): Path<Uuid>,
) -> Result<axum::Json<serde_json::Value>, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let imgs: Vec<Image> = images::table
        .filter(images::item_id.eq(item_id))
        .load(&mut conn)
        .await?;
    let list: Vec<serde_json::Value> = imgs.iter().map(|i| {
        serde_json::json!({
            "ImageType": i.image_type,
            "ImageTag": i.tag,
            "Width": i.width,
            "Height": i.height,
        })
    }).collect();
    Ok(axum::Json(serde_json::json!(list)))
}

/// GET /Items/{itemId}/Images/{imageType}
async fn get_item_image(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path((item_id, image_type)): Path<(Uuid, String)>,
    Query(_params): Query<ImageQuery>,
) -> Result<Response, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;

    // Try direct image lookup.
    let img: Option<Image> = images::table
        .filter(images::item_id.eq(item_id))
        .filter(images::image_type.eq(&image_type))
        .first(&mut conn)
        .await
        .optional()?;

    if let Some(img) = img {
        return serve_image(&img.file_path).await;
    }

    // Track fallback: serve the track's album image instead.
    let album_id: Option<Uuid> = tracks::table
        .filter(tracks::id.eq(item_id))
        .select(tracks::album_id)
        .first::<Option<Uuid>>(&mut conn)
        .await
        .optional()?
        .flatten();

    if let Some(aid) = album_id {
        let album_img: Option<Image> = images::table
            .filter(images::item_id.eq(aid))
            .filter(images::image_type.eq(&image_type))
            .first(&mut conn)
            .await
            .optional()?;
        if let Some(img) = album_img {
            return serve_image(&img.file_path).await;
        }
    }

    Err(ApiError::NotFound("Image not found".to_string()))
}

/// GET /Artists/{name}/Images/{imageType}/{index}
async fn get_artist_image(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path((name, image_type, _index)): Path<(String, String, u32)>,
) -> Result<Response, ApiError> {
    let mut conn = state.pool.get().await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let artist_id: Option<Uuid> = artists::table
        .filter(artists::name.eq(&name))
        .select(artists::id)
        .first(&mut conn)
        .await
        .optional()?;
    let Some(aid) = artist_id else {
        return Err(ApiError::NotFound("Artist not found".to_string()));
    };
    let img: Image = images::table
        .filter(images::item_id.eq(aid))
        .filter(images::image_type.eq(&image_type))
        .first(&mut conn)
        .await
        .optional()?
        .ok_or_else(|| ApiError::NotFound("Image not found".to_string()))?;
    serve_image(&img.file_path).await
}

/// GET /MusicGenres/{name}/Images/{imageType}/{index} — always 404 (no genre art)
async fn get_genre_image(
    _auth: AuthUser,
    _state: State<AppState>,
    _path: Path<(String, String, u32)>,
) -> StatusCode {
    StatusCode::NOT_FOUND
}
