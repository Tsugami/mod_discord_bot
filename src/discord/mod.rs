mod commands;
mod custom_id;

pub mod event_handler;

use poise::{
    serenity_prelude::{self as serenity, GatewayIntents},
    PrefixFrameworkOptions,
};

use crate::bot_context::BotContext;

use self::{
    commands::{register, voices},
    event_handler::handle_listener,
};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
pub struct Data {
    pub ctx: BotContext,
}

pub struct DiscordBot {}

impl DiscordBot {
    pub async fn run(ctx: BotContext) -> Result<(), serenity::Error> {
        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::GUILD_VOICE_STATES
            | GatewayIntents::MESSAGE_CONTENT;

        let framework = poise::Framework::builder()
            .options(poise::FrameworkOptions {
                prefix_options: PrefixFrameworkOptions {
                    prefix: Some("!".into()),
                    ..Default::default()
                },
                commands: vec![voices(), register()],
                listener: |ctx, event, framework, state| {
                    Box::pin(handle_listener(&ctx, event, framework, state))
                },
                ..Default::default()
            })
            .token(&ctx.config.discord_token)
            .intents(intents)
            .user_data_setup(move |_ctx, _ready, _framework| {
                Box::pin(async move { Ok(Data { ctx }) })
            });

        framework.run().await
    }
}
