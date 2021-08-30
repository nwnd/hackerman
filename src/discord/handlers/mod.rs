pub mod dns;
pub mod gitsnip;
pub mod imgify;
pub mod minecraft;
pub mod rfc;
pub mod self_info;
pub mod uwu;
pub mod fakepng;
pub mod rickroll;
pub mod gtfo;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandHandlerError {
    #[error("Command Ignored")]
    Ignore,
    #[error("Command Not Found {0}")]
    UnknownCommand(String),
    #[error(transparent)]
    CodeblockError(#[from] git2codeblock::Error),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    RipplError(#[from] rippl::error::Error),
    #[error(transparent)]
    SongbirdError(#[from] songbird::input::error::Error),
}
