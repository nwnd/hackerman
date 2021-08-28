use uwuifier::uwuify_str_sse;

use super::CommandHandlerError;


pub async fn handle_uwu(text: String) -> Result<String, CommandHandlerError> {
    Ok(uwuify_str_sse(text.as_str()))
}