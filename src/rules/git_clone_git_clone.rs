use super::{Command, Rule};

pub struct GitCloneGitClone;

impl Rule for GitCloneGitClone {
    fn name(&self) -> &str { "git_clone_git_clone" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
