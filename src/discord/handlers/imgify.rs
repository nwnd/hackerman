use serenity::{client::Context, model::channel::Message, Error};

use super::CommandHandlerError;

pub async fn imgify_command(message: String) -> Result<String, CommandHandlerError> {
    // Respond with an image link
    Ok(format!(
        "https://dummyimage.com/256x256/fff.png&text={}",
        urlencoding::encode(&message)
    ))
}
