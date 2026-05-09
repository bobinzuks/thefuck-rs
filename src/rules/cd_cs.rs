use super::{Command, Rule};

pub struct CsToCd;

impl Rule for CsToCd {
    fn match(&self, command: &Command) -> bool {
        command.script_parts.first().map_or(false, |s| s == "cs")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        vec!["cd".to_string() + &command.script[2..]]
    }

    fn priority(&self) -> u32 {
        900
    }
}
