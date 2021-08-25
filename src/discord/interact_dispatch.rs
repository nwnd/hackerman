use serenity::{
    builder::CreateEmbed,
    model::{channel::Embed, interactions::application_command::ApplicationCommandInteraction},
};
use tracing::info;

use crate::discord::{cmdutil::get_nth_string_from_command_data, handlers::{dns::{lookup_dns, lookup_dns_reverse}, gitsnip::handle_gitsnip, imgify::imgify_command, minecraft::lookup_mc_server, rfc::lookup_rfc, self_info::get_self_info}};

use super::{event_handler::Handler, handlers::CommandHandlerError};

pub struct RichRepsonse {
    pub body: Option<String>,
    pub embed: Option<CreateEmbed>,
}

impl From<String> for RichRepsonse {
    fn from(body: String) -> Self {
        Self {
            body: Some(body),
            embed: None,
        }
    }
}

/// Called on incoming command
pub async fn dispatch_command(
    handler: &Handler,
    command: &ApplicationCommandInteraction,
) -> Option<Result<RichRepsonse, CommandHandlerError>> {
    // Command name
    let cmd_name = command.data.name.as_str();
    info!("Got command {} from user {}", cmd_name, command.user);

    // Check if the command is a nologic command
    for cmd in &handler.cmdmap.nologic {
        if cmd.name == cmd_name {
            // Return the response right away since this is a nologic cmd
            return Some(Ok(cmd.response.clone().into()));
        }
    }

    // Handle anything with logic attached
    if let Some(response) = match cmd_name {
        "rfc" => Some(
            lookup_rfc(
                get_nth_string_from_command_data(&command, 0)
                    .unwrap_or(String::new())
                    .parse()
                    .unwrap(),
            )
            .await,
        ),
        "imgify" => Some(
            imgify_command(get_nth_string_from_command_data(&command, 0).unwrap())
                .await
                .map(|s| s.into()),
        ),
        "dns" => Some(
            lookup_dns(get_nth_string_from_command_data(&command, 0).unwrap())
                .await
                .map(|s| s.into()),
        ),
        "rdns" => Some(
            lookup_dns_reverse(get_nth_string_from_command_data(&command, 0).unwrap())
                .await
                .map(|s| s.into()),
        ),
        "mc_lookup" => Some(
            match get_nth_string_from_command_data(&command, 1)
                .unwrap_or("25565".to_string())
                .parse()
            {
                Ok(port) => {
                    lookup_mc_server(get_nth_string_from_command_data(&command, 0).unwrap(), port)
                        .await
                        .map(|s| s.into())
                }
                Err(_) => Ok("Invalid port number specified".to_string().into()),
            },
        ),
        "self" => Some(
            get_self_info(&command.user, &command.member)
                .await
                .map(|s| s.into()),
        ),
        "gitsnip" => Some(
            handle_gitsnip(get_nth_string_from_command_data(&command, 0).unwrap())
                .await
                .map(|s| s.into()),
        ),
        _ => None,
    } {
        return Some(response);
    }

    // Handler for unimplemented commands
    Some(Err(CommandHandlerError::UnknownCommand(format!(
        "Command unknown or not implemented: {}",
        cmd_name
    ))))
}
