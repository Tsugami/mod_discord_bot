use super::{custom_id::InteractionCustomId, Data};
use crate::discord::{Context, Error};
use poise::serenity_prelude::{
    self as serenity, ButtonStyle, CreateActionRow, CreateButton, GuildId, ReactionType,
};

fn button(custom_id: &str, emoji: ReactionType) -> CreateButton {
    let mut b = CreateButton::default();
    b.custom_id(custom_id);
    b.emoji(emoji);
    b.style(ButtonStyle::Primary);

    b
}

const LIMIT: i64 = 10;

pub struct VoiceCommandMessage {
    pub content: String,
    pub component_row: CreateActionRow,
}

pub async fn build_voice_message(
    guild_id: &GuildId,
    user_data: &Data,
    user_id: &serenity::UserId,
    page: i64,
) -> Result<VoiceCommandMessage, Error> {
    let skip = (page - 1) * LIMIT;

    let data = user_data
        .ctx
        .database
        .get_voice_states(crate::database::VoiceStateUpdatePaginationInput {
            guild_id: guild_id.to_string(),
            user_id: user_id.to_string(),
            limit: LIMIT as i64,
            skip: Some(skip),
        })
        .await?;

    let mut content = format!(
        "{}'s account. {} voice connections.\n",
        &user_id, data.count
    );

    for i in data.data {
        let channel_message = match i.channel_id {
            Some(new_ch_id) => match i.old_channel_id {
                None => {
                    format!("<:voicejoin:1033909980631412747> <#{}>", new_ch_id)
                }
                Some(old_ch_id) => {
                    format!(
                        "<:voiceswitch:1033909979498946631> <#{}> `->` <#{}>",
                        old_ch_id, new_ch_id
                    )
                }
            },
            None => {
                format!(
                    "<:voiceleave:1033909982678237225> <#{}>",
                    i.old_channel_id.unwrap()
                )
            }
        };

        let timestamp = i.created_at.assume_utc().unix_timestamp().to_string();
        content.push_str(channel_message.as_str());
        content.push_str(format!(" - <t:{}:f> (<t:{}:R>)", &timestamp, &timestamp).as_ref());
        content.push_str("\n");
    }

    let mut row = CreateActionRow::default();

    let page_count = (data.count as f64 / LIMIT as f64).ceil() as i64;
    let has_next_page = page < page_count;
    let has_previous_page = page != 1;
    println!(
        "page_count {}, page {}, limit {},skip {}",
        page_count, page, LIMIT, skip
    );

    row.add_button(button(
        &InteractionCustomId::Page {
            user_id: user_id.to_owned(),
            page: 1,
        }
        .to_string("home"),
        "🔃".parse().unwrap(),
    ));

    if has_previous_page {
        row.add_button(button(
            &InteractionCustomId::Page {
                user_id: user_id.to_owned(),
                page: page - 1,
            }
            .to_string("previous_page"),
            "◀️".parse().unwrap(),
        ));
    }

    if has_next_page {
        row.add_button(button(
            &InteractionCustomId::Page {
                user_id: user_id.to_owned(),
                page: page + 1,
            }
            .to_string("next_page"),
            "▶️".parse().unwrap(),
        ));
    }

    Ok(VoiceCommandMessage {
        component_row: row.to_owned(),
        content: content.to_owned(),
    })
}

#[poise::command(slash_command, prefix_command, guild_only, aliases("voice", "v"))]
pub async fn voices(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let data = build_voice_message(
        &ctx.guild_id().unwrap(),
        ctx.framework().user_data,
        &u.id,
        1,
    )
    .await?;

    ctx.send(|m| {
        m.content(data.content)
            .components(|c| c.add_action_row(data.component_row))
    })
    .await?;
    Ok(())
}

#[poise::command(prefix_command, check = "is_owner")]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

async fn is_owner(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx.author().id.to_string() == ctx.framework().user_data.ctx.config.owner_id)
}
