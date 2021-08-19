use chrono::{DateTime, NaiveDateTime, Utc};
use serenity::model::{guild::Member, prelude::User};

use super::CommandHandlerError;

pub async fn get_self_info(
    user: &User,
    member: &Option<Member>,
) -> Result<String, CommandHandlerError> {
    Ok(format!(
        "**User Info For: `{}#{}`**\n\nUser ID: `{}`\nBot: `{}`{}",
        user.name,
        user.discriminator,
        user.id.to_string(),
        user.bot,
        match member {
            Some(member) => format!(
                "\nDeaf: `{}`\nJoined: `{}`\nMute: `{}`\nNickname: `{}`",
                member.deaf,
                member
                    .joined_at
                    .unwrap_or(DateTime::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)),
                member.mute,
                member.nick.as_ref().unwrap_or(&user.name)
            ),
            None => "".to_string(),
        }
    ))
}
