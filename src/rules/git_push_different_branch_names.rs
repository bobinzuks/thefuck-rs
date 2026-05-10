use super::{Command, Rule};

pub struct GitPushDifferentBranchNames;

impl Rule for GitPushDifferentBranchNames {
    fn name(&self) -> &str { "git_push_different_branch_names" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
