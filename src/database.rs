use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config::Config;

pub struct Database {
    pool: Pool<Postgres>,
}

pub struct CreateVoiceStateUpdateInput {
    pub channel_id: Option<String>,
    pub old_channel_id: Option<String>,
    pub guild_id: String,
    pub user_id: String,
}

impl Database {
    pub async fn new(config: &Config) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await?;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Couldn't run database migrations");

        Ok(Database { pool })
    }

    pub async fn create_voice_state_update(
        &self,
        input: CreateVoiceStateUpdateInput,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            INSERT INTO voice_state_update (
                channel_id, user_id, guild_id, old_channel_id
            ) VALUES ($1, $2, $3, $4)
        ",
            input.channel_id,
            input.user_id,
            input.guild_id,
            input.old_channel_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
