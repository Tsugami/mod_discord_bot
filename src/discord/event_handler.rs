use poise::serenity_prelude::{self as serenity, interaction, Interaction};
use poise::serenity_prelude::{Ready, VoiceState};

use crate::{bot_context::BotContext, database};

use super::{Data, Error};

pub async fn handle_listener(
    _ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    user_data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::VoiceStateUpdate { new } => {
            voice_state_update_event(&user_data.ctx, new.to_owned()).await;
        }

        poise::Event::Ready { data_about_bot } => {
            ready_event(data_about_bot.to_owned());
        }

        poise::Event::InteractionCreate {
            interaction: Interaction::MessageComponent(interaction),
        } => {
            println!("interaction {:?}", interaction)
        }
        _ => (),
    };

    Ok(())
}

async fn voice_state_update_event(ctx: &BotContext, voice_state: VoiceState) {
    let user_id = voice_state.user_id.to_string();

    let old_channel_id = ctx.voice_connections.get(&user_id).await.unwrap();

    match voice_state.channel_id {
        None => {
            ctx.voice_connections.rem(&user_id).await.unwrap();

            if let Some(_) = old_channel_id {
                ctx.database
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
            ctx.voice_connections
                .set(&user_id.to_string(), &new_channel_id.to_string())
                .await
                .unwrap();

            ctx.database
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

fn ready_event(ready: Ready) {
    println!("{} is connected!", ready.user.name);
}
