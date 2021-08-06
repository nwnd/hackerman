pub mod imgify;
pub mod dns;
pub mod minecraft;

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
