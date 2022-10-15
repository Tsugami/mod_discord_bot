use dotenv;

use figment::{providers::Env, Error, Figment};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub discord_token: String,
    pub database_url: String,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        dotenv::dotenv().ok();

        let config: Config = Figment::new().merge(Env::prefixed("MOD_")).extract()?;

        return Ok(config);
    }
}
