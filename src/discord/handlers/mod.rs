pub mod dns;
pub mod gitsnip;
pub mod imgify;
pub mod minecraft;
pub mod rfc;
pub mod self_info;
pub mod uwu;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandHandlerError {
    #[error("Command Ignored")]
    Ignore,
    #[error("Command Not Found {0}")]
    UnknownCommand(String),
    #[error(transparent)]
    CodeblockError(git2codeblock::Error),
}
