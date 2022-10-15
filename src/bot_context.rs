use std::{collections::HashMap, hash::Hash};

use serenity::model::prelude::{ChannelId, UserId};

use crate::{config::Config, database::Database};

pub struct BotContext {
    pub config: Config,
    pub database: Database,
    pub voice_connections: VoiceConnectionCache,
}

pub struct VoiceConnectionCache {
    data: HashMap<UserId, ChannelId>,
}

impl VoiceConnectionCache {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, user_id: &UserId) -> Option<&ChannelId> {
        self.data.get(&user_id)
    }

    pub fn set(&mut self, user_id: UserId, channel_id: ChannelId) -> () {
        match self.data.insert(user_id, channel_id) {
            _ => (),
        }
    }

    pub fn delete(&mut self, user_id: &UserId) -> () {
        match self.data.remove(user_id) {
            _ => (),
        }
    }
}

impl BotContext {
    pub fn new(config: Config, database: Database) -> Self {
        Self {
            config,
            database,
            voice_connections: VoiceConnectionCache::new(),
        }
    }
}
