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
pub mod scanner;

#[cfg(feature = "server")]
pub use db::{create_pool, run_migrations};
#[cfg(feature = "server")]
pub use routes::create_router;
#[cfg(feature = "server")]
pub use state::AppState;
#[cfg(feature = "server")]
pub use scanner::{full_scan, quick_scan};

/// Create default admin user if the users table is empty (first-run bootstrap).
#[cfg(feature = "server")]
pub async fn bootstrap(state: &AppState) {
    use auth::password;
    use db::{
        models::NewUser,
        schema::users,
    };
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use uuid::Uuid;

    let Ok(mut conn) = state.pool.get().await else { return };

    let count: i64 = users::table
        .count()
        .get_result(&mut conn)
        .await
        .unwrap_or(1); // default to 1 so we don't create a user on DB error

    if count > 0 {
        return;
    }

    let admin_user = std::env::var("DIOXUS_MUSIC_ADMIN_USER")
        .unwrap_or_else(|_| "admin".to_string());
    let admin_pass = std::env::var("DIOXUS_MUSIC_ADMIN_PASSWORD")
        .unwrap_or_else(|_| "changeme".to_string());

    let hash = match password::hash_password(&admin_pass) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("Failed to hash admin password: {e}");
            return;
        }
    };

    let new_user = NewUser {
        id: Uuid::new_v4(),
        name: admin_user.clone(),
        password_hash: hash,
        is_admin: true,
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
        .await
    {
        Ok(_) => tracing::info!("Created default admin user '{admin_user}'"),
        Err(e) => tracing::error!("Failed to create admin user: {e}"),
    }
}
