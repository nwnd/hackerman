use async_minecraft_ping::ConnectionConfig;
use serenity::model::connection;

use super::CommandHandlerError;

pub async fn lookup_mc_server(address: String, port: u16) -> Result<String, CommandHandlerError> {
    // Build connection config
    let config = ConnectionConfig::build(&address).with_port(port);

    // Connect
    let mut connection = config.connect().await.unwrap();
    let status = connection.status().await.unwrap();

    // Respond with an image link
    Ok(format!(
        "**Minecraft Server `{}` ({})**\n*{}/{} players online*",
        format!("{}:{}", address, port),
        status.version.name,
        status.players.online,
        status.players.max
    ))
}
