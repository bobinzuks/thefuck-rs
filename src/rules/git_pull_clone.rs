use super::{Command, Rule};

pub struct GitPullClone;

impl Rule for GitPullClone {
    fn name(&self) -> &str { "git_pull_clone" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
