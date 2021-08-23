pub mod imgify;
pub mod dns;
pub mod minecraft;
pub mod self_info;

#[derive(Debug)]
pub enum CommandHandlerError {
    Ignore,
    UnknownCommand(String)
}
