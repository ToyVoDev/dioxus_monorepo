pub mod types;

#[cfg(feature = "server")]
pub mod auth;
#[cfg(feature = "server")]
pub mod db;
#[cfg(feature = "server")]
pub mod error;
#[cfg(feature = "server")]
pub mod routes;
#[cfg(feature = "server")]
pub mod state;

#[cfg(feature = "server")]
pub use state::AppState;
#[cfg(feature = "server")]
pub use routes::create_router;
#[cfg(feature = "server")]
pub use db::{create_pool, run_migrations};

/// Create default admin user if no users exist.
#[cfg(feature = "server")]
pub async fn bootstrap(_state: &AppState) {
    // implemented in Task 12
}
