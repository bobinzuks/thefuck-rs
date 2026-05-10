use super::{Command, Rule};

pub struct GitRmLocalModifications;

impl Rule for GitRmLocalModifications {
    fn name(&self) -> &str { "git_rm_local_modifications" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
