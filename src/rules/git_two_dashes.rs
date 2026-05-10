use super::{Command, Rule};

pub struct GitTwoDashes;

impl Rule for GitTwoDashes {
    fn name(&self) -> &str { "git_two_dashes" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
