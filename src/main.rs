use bot_context::BotContext;
use database::Database;
use discord::DiscordBot;

mod bot_context;
mod config;
mod database;
mod discord;
mod util;
mod voice_connections;

#[tokio::main]
async fn main() {
    let config = config::Config::load().unwrap();
    let db = Database::new(&config).await.unwrap();
    let ctx = BotContext::new(config, db);

    if let Err(why) = DiscordBot::run(ctx).await {
        println!("Client error: {:?}", why);
    }
}
