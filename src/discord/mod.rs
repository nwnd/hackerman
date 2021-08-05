use serenity::Client;

mod event_handler;
mod handlers;

/// Build and configure a bot client
pub async fn build_bot_client(token: &str, app_id: u64) -> Client {
    Client::builder(token)
        .event_handler(event_handler::Handler)
        .application_id(app_id)
        .await
        .unwrap()
}
