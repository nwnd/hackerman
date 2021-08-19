//! Bot application entrypoint

use std::env;

use clap::{App, Arg, crate_authors, crate_description, crate_name, crate_version};

use crate::discord::{build_bot_client, cmdmap::CmdMap};
use tracing::instrument;

mod discord;

#[tokio::main]
async fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(
            Arg::with_name("cmdmap")
                .help("Path to a cmdmap.json file")
                .required(true),
        )
        .get_matches();

    // Enable logging
    tracing_subscriber::fmt::init();

    // Load the cmdmap
    let cmdmap_path = matches.value_of("cmdmap").unwrap();
    let cmdmap: CmdMap = autojson::structify(cmdmap_path).unwrap();

    // Get the discord tokens
    let discord_token = env::var("DISCORD_TOKEN").expect("$DISCORD_TOKEN not set");
    let discord_app_id: u64 = env::var("DISCORD_APP_ID")
        .expect("$DISCORD_APP_ID not set")
        .parse()
        .expect("Application ID is not valid");

    // Set up the discord client
    let mut discord_client = build_bot_client(discord_token.as_str(), discord_app_id, cmdmap).await;

    // Start the client
    if let Err(why) = discord_client.start().await {
        println!("Client error: {:?}", why);
    }
}
