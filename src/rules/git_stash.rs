use super::{Command, Rule};

pub struct GitStash;

impl Rule for GitStash {
    fn name(&self) -> &'static str {
        "git_stash"
    }

    fn matches(&self, command: &Command) -> bool {
        command.output.contains("or stash them")
    }

    fn fix(&self, command: &Command) -> String {
        format!("git stash && {}", command.text)
    }
}