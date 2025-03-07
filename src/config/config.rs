use sqlx::postgres::PgPoolOptions;
use std::env;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
        })
    }

    pub async fn db_pool(&self) -> sqlx::Pool<sqlx::Postgres> {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.database_url)
            .await
            .expect("Failed to create pool.")
    }
}
