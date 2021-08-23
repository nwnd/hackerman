pub mod imgify;
pub mod dns;
pub mod minecraft;
pub mod self_info;
pub mod rfc;

#[derive(Debug)]
pub enum CommandHandlerError {
    Ignore,
    LibError(failure::Error),
    UnknownCommand(String)
}

impl From<failure::Error> for CommandHandlerError {
    fn from(e: failure::Error) -> Self {
        CommandHandlerError::LibError(e)
    }
}
