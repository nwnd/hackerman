use crate::discord::handlers::{self_info::get_self_info};

use super::handlers::{
    dns::{lookup_dns, lookup_dns_reverse},
    imgify::imgify_command,
    minecraft::lookup_mc_server,
};

use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::{
                ApplicationCommand, ApplicationCommandInteraction,
                 ApplicationCommandOptionType,
            },
            Interaction, InteractionResponseType,
        },
        prelude::Activity,
    },
    prelude::*,
};

// Authorized guilds
const NWND_GUILD_ID: u64 = 717476650014736394;

fn get_nth_string_from_command_data(
    command: &ApplicationCommandInteraction,
    nth: usize,
) -> Option<String> {
    match command.data.options.get(nth) {
        Some(option) => match option.value.as_ref() {
            Some(val) => match val.as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            None => None,
        },
        None => None,
    }
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Got command: {:?}", command.data.name);

            let content = match command.data.name.as_str() {
                "hackerping" => Some(Ok("***HACKERMAN* is here!**".to_string())),
                "imgify" => Some(
                    imgify_command(
                        get_nth_string_from_command_data(&command, 0).unwrap(),
                    )
                    .await
                ),
                "hackerman" => Some(Ok("https://i.kym-cdn.com/entries/icons/original/000/021/807/ig9OoyenpxqdCQyABmOQBZDI0duHk2QZZmWg2Hxd4ro.jpg".to_string())),
                "dns" => Some(
                    lookup_dns(
                        get_nth_string_from_command_data(&command, 0).unwrap(),
                    )
                    .await
                ),
                "rdns" => Some(
                    lookup_dns_reverse(
                        get_nth_string_from_command_data(&command, 0).unwrap(),
                    )
                    .await
                ),
                "irc" => Some(Ok("You can connect in to this server over IRC!\n\nAddress: `nerds.irc.retrylife.ca`\nPort: `6667` (no SSL)\n\nPing Evan for the password.".to_string())),
                "mc_lookup" => Some(
                    match get_nth_string_from_command_data(&command, 1).unwrap_or("25565".to_string()).parse() {
                        Ok(port) => lookup_mc_server(
                            get_nth_string_from_command_data(&command, 0).unwrap(),
                            port,
                        )
                        .await,
                        Err(_) => Ok("Invalid port number specified".to_string())
                    }
                    
                ),
                "self" => Some(
                    get_self_info(
                        &command.user,
                         &command.member
                    )
                    .await
                ),
                "funni" => Some(Ok("https://i.imgur.com/l4L5T3g.jpg".to_string())),
                "coordsystems" => Some(Ok("https://i.imgur.com/KJVw88H.jpg".to_string())),
                _ => None,
            };

            // Only respond to valid commands
            if let Some(result) = content {
                // Handle possible errors
                match result {
                    Ok(resp) => {
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
                    Err(err) => match err {
                        super::handlers::CommandHandlerError::Ignore => {}
                        _ => {
                            if let Err(why) = command
                                .create_interaction_response(&ctx.http, |response| {
                                    response
                                        .kind(InteractionResponseType::ChannelMessageWithSource)
                                        .interaction_response_data(|message| {
                                            message.content(format!(
                                                "A server error occured: \n```\n{:#?}\n```",
                                                err
                                            ))
                                        })
                                })
                                .await
                            {
                                println!("Cannot respond to slash command: {}", why);
                            }
                        }
                    },
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // Set up global commands
        let commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            // Hackerping command
            commands.create_application_command(|command| {
                command
                    .name("hackerping")
                    .description("Check HACKERMAN's status")
            })
            // Hackerman himself
            .create_application_command( |command| {
                command
                    .name("hackerman")
                    .description("Experience the one and only")
            })
            // imgify command
            .create_application_command(|command| {
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
            // DNS lookup
            .create_application_command(|command| {
                command
                    .name("dns")
                    .description("Perform a DNS lookup")
                    .create_option(|option| {
                        option
                            .name("domain")
                            .description("Domain name to look up")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                    })
            })
            // Reverse DNS lookup
            .create_application_command( |command| {
                command
                    .name("rdns")
                    .description("Perform a reverse DNS lookup")
                    .create_option(|option| {
                        option
                            .name("ip")
                            .description("Ip address to look up")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                    })
            })
            // Minecraft server lookup
            .create_application_command( |command| {
                command
                    .name("mc_lookup")
                    .description("Query a Minecraft server")
                    .create_option(|option| {
                        option
                            .name("address")
                            .description("Minecraft server address")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                    })
                    .create_option(|option| {
                        option
                            .name("port")
                            .description("Minecraft server port")
                            .kind(ApplicationCommandOptionType::Integer)
                            .required(false)
                    })
            })
            // Self info
            .create_application_command(|command| {
                command
                    .name("self")
                    .description("Get information about yourself")
            })
            // Funni command
            .create_application_command(|command| {
                command
                    .name("funni")
                    .description("Ha")
            })
            // coordinates command
            .create_application_command(|command| {
                command
                    .name("coordsystems")
                    .description("Coordinate system reference")
            })
        }).await.unwrap();

        // IRC command
        GuildId(NWND_GUILD_ID)
            .create_application_command(&ctx.http, |command| {
                command
                    .name("irc")
                    .description("Get information about the IRC server")
            })
            .await
            .unwrap();

        println!("Commands configured.");

        // Set the activity
        ctx.set_activity(Activity::watching("over script kiddies"))
            .await;
    }
}
