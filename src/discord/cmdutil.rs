use serenity::model::interactions::application_command::ApplicationCommandInteraction;


/// Gets the nth string from a command's argument data
pub fn get_nth_string_from_command_data(
    command: &ApplicationCommandInteraction,
    nth: usize,
) -> Option<String> {
    match command.data.options.get(nth) {
        Some(option) => match option.value.as_ref() {
            Some(val) => match val.as_str() {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            None => None,
        },
        None => None,
    }
}