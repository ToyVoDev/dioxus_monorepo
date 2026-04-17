pub mod models;
pub mod schema;

use diesel::PgConnection;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8;

pub type DbPool = bb8::Pool<AsyncPgConnection>;

pub async fn create_pool(database_url: &str) -> DbPool {
    let config =
        diesel_async::pooled_connection::AsyncDieselConnectionManager::<AsyncPgConnection>::new(
            database_url,
        );
    bb8::Pool::builder()
        .build(config)
        .await
        .expect("Failed to create database connection pool")
}

pub fn run_migrations(database_url: &str) {
    use diesel::Connection;
    use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    let mut conn = PgConnection::establish(database_url)
        .expect("Failed to connect to database for migrations");

    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run database migrations");
}
