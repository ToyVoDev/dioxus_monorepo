pub mod albums;
pub mod artists;
pub mod audio;
pub mod custom;
pub mod genres;
pub mod images;
pub mod items;
pub mod query;
pub mod search;
pub mod users;

use axum::Router;
use crate::state::AppState;

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
        .merge(custom::router())
        .with_state(state)
}
