use super::{cmdmap::CmdMap, interact_dispatch::dispatch_command};

use cfg_if::cfg_if;
use serenity::{
    async_trait,
    model::{
        channel::Embed,
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::{ApplicationCommand, ApplicationCommandOptionType},
            Interaction, InteractionResponseType,
        },
        prelude::Activity,
    },
    prelude::*,
};
use tracing::{error, info};

pub struct Handler {
    pub cmdmap: CmdMap,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // Dispatch the command
            let response = dispatch_command(&self, &command).await;

            // Only respond to valid commands
            if let Some(result) = response {
                // Handle possible errors
                match result {
                    Ok(resp) => {
                        if let Err(why) = command
                            .create_interaction_response(&ctx.http, |response| {
                                response
                                    .kind(InteractionResponseType::ChannelMessageWithSource)
                                    .interaction_response_data(|message| {
                                        let mut m = message;
                                        if let Some(body) = resp.body {
                                            m = m.content(body);
                                        }
                                        if let Some(embed) = resp.embed {
                                            m = m.add_embed(embed);
                                        }
                                        m
                                    })
                            })
                            .await
                        {
                            error!("Cannot respond to slash command: {}", why);
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
                                error!("Cannot respond to slash command: {}", why);
                            }
                        }
                    },
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        // Set up global commands
        let _commands =
            ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
                // Handle configuring any JSON commands
                info!("Loading commands from JSON");
                let mut cmds = commands;

                // Handle logic-less commands
                for cmd in &self.cmdmap.nologic {
                    if cmd.scope.is_none() {
                        info!("Setting up global command: /{}", cmd.name);
                        cmds = cmds.create_application_command(|command| {
                            command
                                .name(cmd.name.as_str())
                                .description(cmd.description.as_str())
                        });
                    }
                }

                // Handle commands with logic
                for cmd in &self.cmdmap.logic {
                    if cmd.scope.is_none() {
                        info!("Setting up global command: /{}", cmd.name);
                        cmds = cmds.create_application_command(|command| {
                            let mut c = command;
                            c = c
                                .name(cmd.name.as_str())
                                .description(cmd.description.as_str());
                            for arg in &cmd.args {
                                c = c.create_option(|option| {
                                    option
                                        .name(arg.name.as_str())
                                        .description(arg.description.as_str())
                                        .required(arg.required)
                                        .kind(match arg.kind.as_str() {
                                            "User" => ApplicationCommandOptionType::User,
                                            "String" => ApplicationCommandOptionType::String,
                                            "Integer" => ApplicationCommandOptionType::Integer,
                                            "Boolean" => ApplicationCommandOptionType::Boolean,
                                            "Channel" => ApplicationCommandOptionType::Channel,
                                            "Role" => ApplicationCommandOptionType::Role,
                                            "Number" => ApplicationCommandOptionType::Number,
                                            _ => ApplicationCommandOptionType::Unknown,
                                        })
                                });
                            }
                            c
                        });
                    }
                }

                // Return ref
                cmds
            })
            .await
            .unwrap();

        // Set up scoped commands
        for cmd in &self.cmdmap.nologic {
            if let Some(scope) = &cmd.scope {
                for guild in scope {
                    info!("Setting up guild command: /{} in {}", cmd.name, guild);
                    GuildId(*guild)
                        .create_application_command(&ctx.http, |command| {
                            command
                                .name(cmd.name.as_str())
                                .description(cmd.description.as_str())
                        })
                        .await
                        .unwrap();
                }
            }
        }
        for cmd in &self.cmdmap.logic {
            if let Some(scope) = &cmd.scope {
                for guild in scope {
                    info!("Setting up guild command: /{} in {}", cmd.name, guild);
                    GuildId(*guild)
                        .create_application_command(&ctx.http, |command| {
                            let mut c = command;
                            c = c
                                .name(cmd.name.as_str())
                                .description(cmd.description.as_str());
                            for arg in &cmd.args {
                                c = c.create_option(|option| {
                                    option
                                        .name(arg.name.as_str())
                                        .description(arg.description.as_str())
                                        .required(arg.required)
                                        .kind(match arg.kind.as_str() {
                                            "User" => ApplicationCommandOptionType::User,
                                            "String" => ApplicationCommandOptionType::String,
                                            "Integer" => ApplicationCommandOptionType::Integer,
                                            "Boolean" => ApplicationCommandOptionType::Boolean,
                                            "Channel" => ApplicationCommandOptionType::Channel,
                                            "Role" => ApplicationCommandOptionType::Role,
                                            "Number" => ApplicationCommandOptionType::Number,
                                            _ => ApplicationCommandOptionType::Unknown,
                                        })
                                });
                            }
                            c
                        })
                        .await
                        .unwrap();
                }
            }
        }
        info!("Commands configured.");

        // Set the activity
        // We are using the DND specifier to show if running in dev mode
        ctx.set_activity(Activity::watching("over script kiddies"))
            .await;
        cfg_if! {
            if #[cfg(debug_assertions)] {
                info!("Setting status to DND since we are in dev mode");
                ctx.dnd().await;
            } else {
                info!("Setting status to online since we are in production");
                ctx.online().await;
            }
        }
    }
}
