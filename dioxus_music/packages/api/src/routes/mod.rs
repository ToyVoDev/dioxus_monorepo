pub mod albums;
pub mod artists;
pub mod items;
pub mod query;
pub mod users;

use axum::Router;
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(users::router())
        .merge(items::router())
        .merge(artists::router())
        .merge(albums::router())
        .with_state(state)
}
