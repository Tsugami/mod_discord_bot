use deadpool_redis::{Config as PoolConfig, Connection, Pool, PoolError, Runtime};
use redis::{AsyncCommands, RedisError};

use crate::config::Config;

pub type ChannelId = String;
pub type UserId = String;

pub struct VoiceConnections {
    pool: Pool,
}

#[derive(Debug)]
pub enum VoiceConnectionError {
    PoolError(PoolError),
    RedisError(RedisError),
}

impl VoiceConnections {
    pub fn new(config: Config) -> Self {
        let pool = PoolConfig::from_url(&config.redis_url)
            .create_pool(Some(Runtime::Tokio1))
            .expect("Failed to create redis pool");

        Self { pool }
    }

    async fn get_connection(&self) -> Result<Connection, VoiceConnectionError> {
        self.pool
            .get()
            .await
            .map_err(VoiceConnectionError::PoolError)
    }

    pub async fn get(&self, user_id: &UserId) -> Result<Option<String>, VoiceConnectionError> {
        let mut conn = self.get_connection().await?;
        conn.get(user_id.to_string())
            .await
            .map_err(VoiceConnectionError::RedisError)
    }

    pub async fn set(
        &self,
        user_id: &UserId,
        channel_id: &ChannelId,
    ) -> Result<(), VoiceConnectionError> {
        let mut conn = self.get_connection().await?;

        let one_day_in_seconds = 86_400;
        conn.set_ex(user_id, channel_id, one_day_in_seconds)
            .await
            .map_err(VoiceConnectionError::RedisError)
    }

    pub async fn rem(&self, user_id: &UserId) -> Result<(), VoiceConnectionError> {
        let mut conn = self.get_connection().await?;

        conn.del(user_id)
            .await
            .map_err(VoiceConnectionError::RedisError)
    }
}
