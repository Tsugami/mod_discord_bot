use crate::{config::Config, database::Database, voice_connections};

pub struct BotContext {
    pub config: Config,
    pub database: Database,
    pub voice_connections: voice_connections::VoiceConnections,
}

impl BotContext {
    pub fn new(config: Config, database: Database) -> Self {
        Self {
            config: config.clone(),
            database,
            voice_connections: voice_connections::VoiceConnections::new(config),
        }
    }
}
