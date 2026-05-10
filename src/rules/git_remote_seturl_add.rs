use super::{Command, Rule};

pub struct GitRemoteSeturlAdd;

impl Rule for GitRemoteSeturlAdd {
    fn name(&self) -> &'static str {
        "git_remote_seturl_add"
    }

    fn matches(&self, command: &Command) -> bool {
        command.text.contains("set-url")
            && command.output.contains("fatal: No such remote")
    }

    fn fix(&self, command: &Command) -> String {
        command.text.replace("set-url", "add")
    }
}