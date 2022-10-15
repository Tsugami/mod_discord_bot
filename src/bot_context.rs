use crate::{config::Config, database::Database};

pub struct BotContext {
    pub config: Config,
    pub database: Database,
}

impl BotContext {
    pub fn new(config: Config, database: Database) -> Self {
        Self { config, database }
    }
}
