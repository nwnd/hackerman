use super::CommandHandlerError;
use crate::discord::interact_dispatch::RichRepsonse;
use serenity::builder::CreateEmbed;

pub async fn handle_gitsnip(url: String) -> Result<RichRepsonse, CommandHandlerError> {
    // Query the codeblock
    let codeblock = git2codeblock::extract_codeblock(&url).await;

    // Build the embed
    if let Ok(codeblock) = codeblock {
        let embed = CreateEmbed::default()
            .title("Git Snippet")
            .url(url)
            .description(codeblock)
            .to_owned();

        Ok(RichRepsonse {
            body: None,
            embed: Some(embed),
            private: false,
        })
    } else {
        Err(CommandHandlerError::CodeblockError(codeblock.unwrap_err()))
    }
}
