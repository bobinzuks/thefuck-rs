use super::{Command, Rule};

pub struct GitFixStash;

impl Rule for GitFixStash {
    fn name(&self) -> &str { "git_fix_stash" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
