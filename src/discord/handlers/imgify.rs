use serenity::{client::Context, model::channel::Message, Error};

pub async fn imgify_command(
    text: String,
    ctx: Context,
    msg: Message,
) -> Result<serenity::model::channel::Message, Error> {
    // Respond with an image link
    msg.channel_id
        .say(
            &ctx.http,
            format!(
                "https://dummyimage.com/256x256/fff.png&text={}",
                urlencoding::encode(text.as_str())
            ),
        )
        .await
}
