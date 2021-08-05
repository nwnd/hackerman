use log::info;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use super::handlers::imgify::imgify_command;

const COMMAND_PREFIX: &str = "%";

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Grab the first segment of the message
        let mut message_parts = msg.content.split(' ').collect::<Vec<&str>>().into_iter();

        // Only read valid messages
        if let Some(command) = message_parts.next() {
            // Only read messages that start with the command prefix
            if command.starts_with(COMMAND_PREFIX) {
                // Remove the prefix char
                let mut cmd_chars = command.chars();
                cmd_chars.next();
                let command: String = cmd_chars.collect();

                // Fetch the rest of the message as the actual content
                let content = message_parts.collect::<Vec<&str>>().join(" ");

                // Log the event
                info!(
                    "Handling command \"{}\" with arguments: {}",
                    command, content
                );

                match command.as_str() {
                    // Imgify command
                    "imgify" => {
                        imgify_command(content, ctx, msg).await.unwrap();
                    }

                    // Skip unknown commands
                    _ => {}
                }
            }
        }

        // // Handle various commands
        // match message_parts.first() {
        //     Some(command) => match command {
        //         // Ping/Pong
        //         "%ping" => {
        //             msg.channel_id.say(&ctx.http, "Pong!").await;
        //         }

        //         // Skip empty or unknown commands
        //         _ => {}
        //     },
        //     None => {}
        // }

        // if msg.content == "!ping" {
        //     if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        //         println!("Error sending message: {:?}", why);
        //     }
        // }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
