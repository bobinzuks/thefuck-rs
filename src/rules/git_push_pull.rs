use super::{Command, Rule};

pub struct GitPushPull;

impl Rule for GitPushPull {
    fn name(&self) -> &str { "git_push_pull" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
