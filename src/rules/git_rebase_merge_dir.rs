use super::{Command, Rule};

pub struct GitRebaseMergeDir;

impl Rule for GitRebaseMergeDir {
    fn name(&self) -> &str { "git_rebase_merge_dir" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
