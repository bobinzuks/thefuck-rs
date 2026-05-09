use super::{Command, Rule};

pub struct CargoBuild;

impl Rule for CargoBuild {
    fn match(&self, command: &Command) -> bool {
        command.script == "cargo"
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        vec!["cargo build".to_string()]
    }
}
