use super::{Command, Rule};

pub struct GitBranchDeleteCheckedOut;

impl Rule for GitBranchDeleteCheckedOut {
    fn name(&self) -> &str { "git_branch_delete_checked_out" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
