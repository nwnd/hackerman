// use serenity::{client::Context, model::id::GuildId};
use serenity::client::Context;
use serenity::model::guild::Member;
use serenity::model::id::{ChannelId, GuildId};
use songbird::SongbirdKey;

use super::CommandHandlerError;

pub async fn handle_rickroll(
    ctx: &Context,
    guild: &Option<GuildId>,
    channel: &ChannelId,
    member: &Option<Member>
) -> Result<String, CommandHandlerError> {
    // This only works in guilds
    if guild.is_none() {
        return Ok("This command only works in guilds.".to_string());
    }
    let guild = guild.unwrap();

    // Get an audio manager
    let songbird_data = ctx.data.read().await;
    let manager = songbird_data.get::<SongbirdKey>().cloned().expect("Songbird Voice client placed in at initialization.");

    // Get the caller's voice channel

    // let voice_chan = guild.
    // ctx.

    // This only works in voice channels
    if let Some(handler_lock) = manager.get(guild.0) {
        let mut handler = handler_lock.lock().await;
        let source = songbird::ytdl("https://www.youtube.com/watch?v=dQw4w9WgXcQ").await?;
        handler.play_source(source);
    } else {
        return Ok("This command only works in voice channels.".to_string());
    }

    Ok("...".to_string())
}
