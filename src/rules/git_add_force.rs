use super::{Command, Rule};

pub struct GitAddForce;

impl Rule for GitAddForce {
    fn name(&self) -> &str { "git_add_force" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
