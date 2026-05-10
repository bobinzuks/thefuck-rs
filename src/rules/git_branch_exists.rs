use super::{Command, Rule};

pub struct GitBranchExists;

impl Rule for GitBranchExists {
    fn name(&self) -> &str { "git_branch_exists" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
