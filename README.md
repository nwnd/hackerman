# HACKERMAN \[[Add To Server](https://discord.com/api/oauth2/authorize?client_id=872937277779759185&scope=applications.commands%20bot)\]
[![Build](https://github.com/nwnd/hackerman/actions/workflows/build.yml/badge.svg)](https://github.com/nwnd/hackerman/actions/workflows/build.yml)
[![Clippy](https://github.com/nwnd/hackerman/actions/workflows/clippy.yml/badge.svg)](https://github.com/nwnd/hackerman/actions/workflows/clippy.yml)
[![Docker Image](https://github.com/nwnd/hackerman/actions/workflows/docker_img.yml/badge.svg)](https://github.com/nwnd/hackerman/actions/workflows/docker_img.yml)

***HACKERMAN*** is a Discord bot powered by the [Serenity](https://github.com/serenity-rs/serenity) bot framework, written in Rust.

The goal of this bot is to provide a collection of funny memes and useful commands for our private Discord server, along with a few commands for DMs.

## Features

All bot commands are defined in [`cmdmap.json`](./cmdmap.json). The following are the **public** command descriptions:

| Command | Description |
| -- | -- |
| `/hackerping` | Check the bot status |
| `/hackerman` | Make the bot announce itself |
| `/funni` | *Holy shit the bot did a funny thing!* |
| `/coordsystems` | Shows a reference table for the coordinate systems used in animation and game software |
| `/imgify <text>` | Converts text to an image |
| `/dns <domain>` | Performs a DNS lookup on a domain |
| `/rdns <ip>` | Performs a reverse DNS lookup on an IP address |
| `/mc_lookup <address> <port>` | Checks the status of a Minecraft server |
| `/self` | Dumps raw info about the user |
| `/rfc <number>` | Embed an IETF RFC in chat |
| `/gitsnip <url>` | Pulls a file from GitHub or GitLab into a codesnippet |
| `/uwu <text>` | uwu-ify some text |
| `/fakepng <url>` | Commit war crimes to a PNG |

## Building and running

No matter how you run the bot, you must set both the `DISCORD_TOKEN` and `DISCORD_APP_ID` environment variables.

### Running in debug mode

```sh
DISCORD_TOKEN=<token> DISCORD_APP_ID=<app_id> RUST_LOG=info cargo run -- ./cmdmap.json
```

### Running in release mode

```sh
DISCORD_TOKEN=<token> DISCORD_APP_ID=<app_id> RUST_LOG=info cargo run --release -- ./cmdmap.json
```

### Running in Docker

```sh
docker build -t hackerman .
docker run -it --rm -e DISCORD_TOKEN=<token> -e DISCORD_APP_ID=<app_id> hackerman
```

## Screenshots

|                        |                           |
|-----------------------------------|---------------------------|
| ![](assets/hackerman_profile.png) | ![](assets/hackerman.png) |