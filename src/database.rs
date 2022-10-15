use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::path::Path;

use crate::config::Config;

pub struct Database {
    pool: Pool<Postgres>,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "voice_state_update_type")]
#[sqlx(rename_all = "UPPERCASE")]
pub enum VoiceStateUpdateTypes {
    Join,
    Switch,
    Leave,
}

impl VoiceStateUpdateTypes {
    pub fn to_str(self) -> String {
        match self {
            VoiceStateUpdateTypes::Join => "JOIN".to_string(),
            VoiceStateUpdateTypes::Leave => "LEAVE".to_string(),
            VoiceStateUpdateTypes::Switch => "SWITCH".to_string(),
        }
    }
}
pub struct CreateVoiceStateUpdateInput {
    pub channel_id: String,
    pub guild_id: String,
    pub user_id: String,
    pub voice_state_update_type: VoiceStateUpdateTypes,
}

impl Database {
    pub async fn new(config: &Config) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await?;

        // let migration_path_folder = Path::new("./migrations");
        // println!("migrations path: {:?}", migration_path_folder.to_str());

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
                channel_id, user_id, guild_id, type
            ) VALUES ($1, $2, $3, $4)
        ",
            input.channel_id,
            input.user_id,
            input.guild_id,
            input.voice_state_update_type.to_str()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
