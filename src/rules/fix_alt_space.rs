use super::{Command, Rule};

pub struct FixUselessSpace;

impl Rule for FixUselessSpace {
    fn match(&self, command: &Command) -> bool {
        command.output.to_lowercase().contains("command not found")
            && command.script.contains('\u{00a0}')
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        vec![command.script.replace('\u{00a0}', " ")]
    }
}
