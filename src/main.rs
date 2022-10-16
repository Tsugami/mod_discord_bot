use bot_context::BotContext;
use database::Database;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;

mod bot_context;
mod config;
mod database;
mod voice_connections;

struct Handler {
    ctx: BotContext,
}

#[async_trait]
impl EventHandler for Handler {
    async fn voice_state_update(&self, _ctx: Context, voice_state: VoiceState) {
        let user_id = voice_state.user_id.to_string();

        let old_channel_id = self.ctx.voice_connections.get(&user_id).await.unwrap();

        match voice_state.channel_id {
            None => {
                self.ctx.voice_connections.rem(&user_id).await.unwrap();

                if let Some(_) = old_channel_id {
                    self.ctx
                        .database
                        .create_voice_state_update(database::CreateVoiceStateUpdateInput {
                            channel_id: None,
                            guild_id: voice_state.guild_id.unwrap().to_string(),
                            user_id,
                            old_channel_id,
                        })
                        .await
                        .unwrap()
                }
            }
            Some(new_channel_id) => {
                self.ctx
                    .voice_connections
                    .set(&user_id.to_string(), &new_channel_id.to_string())
                    .await
                    .unwrap();

                self.ctx
                    .database
                    .create_voice_state_update(database::CreateVoiceStateUpdateInput {
                        channel_id: Some(new_channel_id.to_string()),
                        guild_id: voice_state.guild_id.unwrap().to_string(),
                        user_id,
                        old_channel_id,
                    })
                    .await
                    .unwrap();
            }
        };
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let config = config::Config::load().unwrap();
    let db = Database::new(&config).await.unwrap();

    let ctx = BotContext::new(config, db);

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&ctx.config.discord_token, intents)
        .event_handler(Handler { ctx })
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
