use super::{Command, Rule};

pub struct AgQuoted;

impl Rule for AgQuoted {
    fn name(&self) -> &'static str {
        "ag_quoted"
    }

    fn is_match(&self, command: &Command) -> bool {
        command.output.ends_with("run ag with -Q\n")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        vec![command.script.replacen("ag", "ag -Q", 1)]
    }
}
