use super::{Command, Rule};

pub struct GitBranchDelete;

impl Rule for GitBranchDelete {
    fn name(&self) -> &str { "git_branch_delete" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
