use crate::discord::interact_dispatch::RichRepsonse;
use super::CommandHandlerError;
use serenity::builder::{CreateEmbed, CreateEmbedAuthor};

pub async fn lookup_rfc(number: u32) -> Result<RichRepsonse, CommandHandlerError> {
    // Fetch the RFC
    let rfc = rfclib::query_rfc(number).await;

    // Build the embed
    if let Ok(rfc) = rfc {
    let embed = CreateEmbed::default()
        .title(format!("RFC {}: ***{}***", number, rfc.title))
        .field("Pages", rfc.pages, true)
        .field("State", rfc.state, true)
        .field("Stream", rfc.stream.unwrap_or("Unknown".to_string()), true)
        .field("View", 
        format!(
            "[TXT](https://www.rfc-editor.org/rfc/rfc{}.txt) | [PDF](https://www.rfc-editor.org/rfc/pdfrfc/rfc{}.txt.pdf) | [HTML](https://www.rfc-editor.org/rfc/rfc{}.html)", 
            number, 
            number, 
            number), 
            false
        )
        .set_author(
            CreateEmbedAuthor::default()
                .name("Internet Engineering Task Force")
                .icon_url("https://www.ietf.org/lib/dt/7.36.0/ietf/images/ietflogo.png")
                .to_owned(),
        )
        .url(format!(
            "https://datatracker.ietf.org/doc/html/rfc{}",
            number
        ))
        .to_owned();

    Ok(RichRepsonse {
        body: None,
        embed: Some(embed),
    })
    } else {
        Ok(format!("RFC `{}` does not exist", number).into())
    }
}
