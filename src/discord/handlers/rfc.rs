use std::ops::Deref;

use crate::discord::interact_dispatch::RichRepsonse;

use super::CommandHandlerError;
use serde::{Deserialize, Serialize};
use serenity::builder::{CreateEmbed, CreateEmbedAuthor};

#[derive(Deserialize, Serialize, Debug)]
struct RfcDoc {
    pub pages: u32,
    pub title: String,
    pub state: String,
    pub stream: String,
}

pub async fn lookup_rfc(number: u32) -> Result<RichRepsonse, CommandHandlerError> {
    // Get metadata about the rfc
    let resp = reqwest::get(format!(
        "https://datatracker.ietf.org/doc/rfc{}/doc.json",
        number
    ))
    .await
    .unwrap();

    // Handle the RFC not existing
    if !resp.status().is_success() {
        return Ok(format!("RFC `{}` does not exist", number).into());
    }

    // Parse the RFC document
    let doc: RfcDoc = resp.json().await.unwrap();

    // Build the embed
    let embed = CreateEmbed::default()
        .title(format!("RFC {}: ***{}***", number, doc.title))
        .field("Pages", doc.pages, true)
        .field("State", doc.state, true)
        .field("Stream", doc.stream, true)
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
}
