use serenity::{
    client::Context,
    model::{channel::Message, guild::Member, prelude::User},
    Error,
};

use super::CommandHandlerError;

pub async fn get_next_spacex_info() -> Result<String, CommandHandlerError> {
    Ok(format!("This command is not yet finished"))
}
