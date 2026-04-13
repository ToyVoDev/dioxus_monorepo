pub mod users;

use axum::Router;
use crate::state::AppState;

pub fn create_router(_state: AppState) -> Router<AppState> {
    Router::new()
        .merge(users::router())
}
