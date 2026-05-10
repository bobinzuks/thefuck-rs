use super::{Command, Rule};

pub struct GitPushForce;

impl Rule for GitPushForce {
    fn name(&self) -> &str { "git_push_force" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
