use super::handlers::imgify::imgify_command;
use log::info;
use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateApplicationCommands},
    model::{
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::{
                ApplicationCommand, ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            Interaction, InteractionResponseType,
        },
    },
    prelude::*,
};

// Authorized guilds
const NWND_GUILD_ID: u64 = 717476650014736394;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "hackerping" => Some("***HACKERMAN* is here!**".to_string()),
                "imgify" => Some(
                    imgify_command(
                        command
                            .data
                            .options
                            .get(0)
                            .expect("Expected some text")
                            .value
                            .as_ref()
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string(),
                    )
                    .await
                    .unwrap(),
                ),
                "hackerman" => Some("https://i.kym-cdn.com/entries/icons/original/000/021/807/ig9OoyenpxqdCQyABmOQBZDI0duHk2QZZmWg2Hxd4ro.jpg".to_string()),
                _ => None,
            };

            // Only respond to valid commands
            if let Some(resp) = content {
                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content(resp))
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // Hackerping command
        GuildId(NWND_GUILD_ID)
            .create_application_command(&ctx.http, |command| {
                command
                    .name("hackerping")
                    .description("Check HACKERMAN's status")
            })
            .await.unwrap();

        // Hackerman himself
        GuildId(NWND_GUILD_ID)
            .create_application_command(&ctx.http, |command| {
                command
                    .name("hackerman")
                    .description("Experience the one and only")
            })
            .await.unwrap();

        // imgify command
        GuildId(NWND_GUILD_ID)
            .create_application_command(&ctx.http, |command| {
                command
                    .name("imgify")
                    .description("Convert text to an image")
                    .create_option(|option| {
                        option
                            .name("text")
                            .description("The message to send")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                    })
            })
            .await.unwrap();

        println!("Commands configured.");
    }
}
