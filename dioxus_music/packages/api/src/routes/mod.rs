pub mod albums;
pub mod artists;
pub mod audio;
pub mod custom;
pub mod genres;
pub mod images;
pub mod items;
pub mod playlists;
pub mod query;
pub mod search;
pub mod sessions;
pub mod user_data;
pub mod users;

use crate::state::AppState;
use axum::Router;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(users::router())
        .merge(items::router())
        .merge(artists::router())
        .merge(albums::router())
        .merge(genres::router())
        .merge(audio::router())
        .merge(images::router())
        .merge(search::router())
        .merge(playlists::router())
        .merge(user_data::router())
        .merge(sessions::router())
        .merge(custom::router())
        .with_state(state)
}
