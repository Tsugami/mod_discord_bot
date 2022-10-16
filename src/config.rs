use dotenv;

#[derive(Clone)]
pub struct Config {
    pub discord_token: String,
    pub database_url: String,
    pub redis_url: String,
    pub owner_id: String,
}

impl Config {
    pub fn load() -> Result<Self, std::env::VarError> {
        dotenv::dotenv().ok();

        let discord_token = std::env::var("DISCORD_TOKEN")?;
        let redis_url = std::env::var("REDIS_URL")?;
        let database_url = std::env::var("DATABASE_URL")?;
        let owner_id = std::env::var("OWNER_ID")?;

        return Ok(Self {
            database_url,
            redis_url,
            owner_id,
            discord_token,
        });
    }
}
