use super::{Command, Rule};

pub struct CdParent;

impl Rule for CdParent {
    fn match_rule(&self, command: &Command) -> bool {
        command.script == "cd.."
    }

    fn get_new_command(&self, command: &Command) -> Option<String> {
        Some("cd ..".to_string())
    }
}
