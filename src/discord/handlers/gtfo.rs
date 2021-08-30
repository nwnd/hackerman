// use serenity::{client::Context, model::id::GuildId};
use serenity::client::Context;
use serenity::model::guild::Member;
use serenity::model::id::{ChannelId, GuildId};
use songbird::SongbirdKey;

use crate::discord::interact_dispatch::RichRepsonse;

use super::CommandHandlerError;

pub async fn handle_gtfo(
    ctx: &Context,
    guild: &Option<GuildId>,
    channel: String,
) -> Result<RichRepsonse, CommandHandlerError> {
    // This only works in guilds
    if guild.is_none() {
        return Ok(RichRepsonse {
            embed: None,
            body: Some("This command only works in guilds.".to_string()),
            private: true,
        });
    }
    let guild = guild.unwrap();

    // Get an audio manager
    let songbird_data = ctx.data.read().await;
    let manager = songbird_data
        .get::<SongbirdKey>()
        .cloned()
        .expect("Songbird Voice client placed in at initialization.");

    // Get the caller's voice channel
    let _handler = manager
        .join(guild, ChannelId(channel.parse().unwrap()))
        .await;

    // This only works in voice channels
    if manager.get(guild.0).is_some() {
        // Leave the VC
        manager.remove(guild.0).await.unwrap();
        Ok(RichRepsonse {
            embed: None,
            body: Some("Goodbye".to_string()),
            private: true,
        })
    } else {
        Ok(RichRepsonse {
            embed: None,
            body: Some("Not in a VC".to_string()),
            private: true,
        })
    }
}
