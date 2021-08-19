use serenity::Client;

use self::cmdmap::CmdMap;

mod event_handler;
mod handlers;
pub mod cmdmap;
mod cmdutil;
mod interact_dispatch;

/// Build and configure a bot client
pub async fn build_bot_client(token: &str, app_id: u64, cmdmap: CmdMap) -> Client {
    Client::builder(token)
        .event_handler(event_handler::Handler {
            cmdmap
        })
        .application_id(app_id)
        .await
        .unwrap()
}
