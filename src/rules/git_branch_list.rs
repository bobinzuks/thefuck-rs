use super::{Command, Rule};

pub struct GitBranchList;

impl Rule for GitBranchList {
    fn name(&self) -> &str { "git_branch_list" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
