pub enum InputCommand {
    Quit,
    Broadcast(String),
    ChangeName(String),
    DiceRoll(String),
}

impl From<String> for InputCommand {
    fn from(value: String) -> Self {
        if !value.starts_with("/") {
            return Self::Broadcast(value);
        }
        match value.split_once(|c: char| c.is_whitespace()) {
            Some((command, argument)) => match command {
                "/nick" => Self::ChangeName(argument.to_owned()),
                "/roll" => Self::DiceRoll(argument.to_owned()),
                _ => Self::Broadcast(value),
            },
            None => match value.as_str() {
                "/quit" => Self::Quit,
                _ => Self::Broadcast(value),
            },
        }
    }
}
