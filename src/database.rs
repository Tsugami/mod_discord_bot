use sqlx::{postgres::PgPoolOptions, types::time::PrimitiveDateTime, Pool, Postgres};

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

pub struct VoiceStateUpdatePaginationInput {
    pub guild_id: String,
    pub user_id: String,
    pub limit: i64,
    pub skip: Option<i64>,
}

pub struct VoiceStateUpdatePaginationData {
    pub channel_id: Option<String>,
    pub old_channel_id: Option<String>,
    pub created_at: PrimitiveDateTime,
}

pub struct VoiceStateUpdatePaginationResult {
    pub count: i64,
    pub data: Vec<VoiceStateUpdatePaginationData>,
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

    pub async fn get_voice_states(
        &self,
        input: VoiceStateUpdatePaginationInput,
    ) -> Result<VoiceStateUpdatePaginationResult, sqlx::Error> {
        let data = sqlx::query_as!(
            VoiceStateUpdatePaginationData,
            "
            SELECT channel_id, old_channel_id, created_at
            FROM voice_state_update
            WHERE guild_id = $1 AND user_id = $2
            ORDER BY created_at desc
            LIMIT $3
            OFFSET $4
        ",
            input.guild_id,
            input.user_id,
            input.limit,
            input.skip.map_or(0, |f| f)
        )
        .fetch_all(&self.pool)
        .await?;

        let count = sqlx::query!(
            "
            SELECT COUNT(*)
            FROM voice_state_update
            WHERE guild_id = $1 AND user_id = $2
        ",
            input.guild_id,
            input.user_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(VoiceStateUpdatePaginationResult {
            count: count.count.map_or(0, |v| v),
            data,
        })
    }
}
