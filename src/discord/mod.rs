use serenity::Client;
use songbird::SerenityInit;
use songbird::Songbird;

use self::cmdmap::CmdMap;

pub mod cmdmap;
mod cmdutil;
mod event_handler;
mod handlers;
mod interact_dispatch;

/// Build and configure a bot client
pub async fn build_bot_client(token: &str, app_id: u64, cmdmap: CmdMap) -> Client {
    let voice = Songbird::serenity();
    Client::builder(token)
        .event_handler(event_handler::Handler { cmdmap })
        .application_id(app_id)
        .register_songbird_with(voice)
        // .voice_manager_arc(voice)
        // .type_map_insert::<SongbirdKey>(voice)
        .await
        .unwrap()
}
