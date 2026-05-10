use super::{Command, Rule};

pub struct GitRemoteDelete;

impl Rule for GitRemoteDelete {
    fn name(&self) -> &'static str {
        "git_remote_delete"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.contains("remote delete") || cmd.output.contains("remote delete")
    }

    fn fix(&self, cmd: &Command) -> String {
        cmd.text.replacen("delete", "remove", 1)
    }
}