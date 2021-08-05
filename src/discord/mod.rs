use serenity::Client;

mod event_handler;

/// Build and configure a bot client
pub async fn build_bot_client(token: &str) -> Client {
    Client::builder(token).event_handler(event_handler::Handler).await.unwrap()
}