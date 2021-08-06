pub mod imgify;
pub mod dns;
pub mod minecraft;
pub mod self_info;
pub mod spacex;

#[derive(Debug)]
pub enum CommandHandlerError {
    Ignore,
    LibError(failure::Error)
}

impl From<failure::Error> for CommandHandlerError {
    fn from(e: failure::Error) -> Self {
        CommandHandlerError::LibError(e)
    }
}
