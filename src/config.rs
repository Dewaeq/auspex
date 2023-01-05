use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct Config {
    pub pool: Pool<Postgres>,
}

impl Config {
    pub async fn new() -> Self {
        let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to create database pool");

        Config { pool }
    }
}
