use super::{Command, Rule};

pub struct GitPull;

impl Rule for GitPull {
    fn name(&self) -> &str { "git_pull" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
