use std::env;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

use crate::discord::build_bot_client;

mod discord;

#[tokio::main]
async fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();

    // Enable logging
    env_logger::init();

    // Get the discord token
    let discord_token = env::var("DISCORD_TOKEN").expect("$DISCORD_TOKEN not set");
    let discord_app_id: u64 = env::var("DISCORD_APP_ID")
        .expect("$DISCORD_APP_ID not set")
        .parse()
        .expect("Application ID is not valid");

    // Set up the discord client
    let mut discord_client = build_bot_client(discord_token.as_str(), discord_app_id).await;

    // Start the client
    if let Err(why) = discord_client.start().await {
        println!("Client error: {:?}", why);
    }
}
