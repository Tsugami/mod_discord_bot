use crate::discord::{Context, Error};
use poise::serenity_prelude as serenity;

const LIMIT: i32 = 10;

#[poise::command(slash_command, prefix_command, guild_only, aliases("voice", "v"))]
pub async fn voices(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let data = ctx
        .framework()
        .user_data
        .ctx
        .database
        .get_voice_states(crate::database::VoiceStateUpdatePaginationInput {
            guild_id: ctx.guild_id().unwrap().to_string(),
            user_id: u.id.to_string(),
            limit: LIMIT as i64,
        })
        .await?;

    let mut response = format!("{}'s account. {} voice connections.\n", u.name, data.count);

    for i in data.data {
        let channel_message = match i.channel_id {
            Some(new_ch_id) => match i.old_channel_id {
                None => {
                    format!("📥 <#{}>", new_ch_id)
                }
                Some(old_ch_id) => {
                    format!("♻️ <#{}> `->` <#{}>", old_ch_id, new_ch_id)
                }
            },
            None => {
                format!("📤 <#{}>", i.old_channel_id.unwrap())
            }
        };

        let timestamp = i.created_at.assume_utc().unix_timestamp().to_string();
        response.push_str(channel_message.as_str());
        response.push_str(format!(" - <t:{}:f> (<t:{}:R>)", &timestamp, &timestamp).as_ref());
        response.push_str("\n");
    }

    ctx.say(response).await?;
    Ok(())
}

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
