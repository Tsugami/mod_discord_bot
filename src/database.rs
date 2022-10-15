use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config::Config;

pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(config: &Config) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await?;

        // Run migrations, which updates the database's schema to the latest version.
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Couldn't run database migrations");

        Ok(Database { pool })
    }
}
