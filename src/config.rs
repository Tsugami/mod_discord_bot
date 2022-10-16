use dotenv;

#[derive(Clone)]
pub struct Config {
    pub discord_token: String,
    pub database_url: String,
}

impl Config {
    pub fn load() -> Result<Self, std::env::VarError> {
        dotenv::dotenv().ok();

        let discord_token = std::env::var("DISCORD_TOKEN")?;
        let database_url = std::env::var("DATABASE_URL")?;

        return Ok(Self {
            database_url,
            discord_token,
        });
    }

    pub fn get_redis_url(self) -> String {
        return "redis://127.0.0.1:6379/".to_string();
    }
}
