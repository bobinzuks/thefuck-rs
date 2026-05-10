use super::{Command, Rule};

pub struct GitBranch0flag;

impl Rule for GitBranch0flag {
    fn name(&self) -> &str { "git_branch_0flag" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
