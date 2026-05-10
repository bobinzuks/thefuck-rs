use super::{Command, Rule};

pub struct GitHookBypass;

const HOOKED_COMMANDS: &[&str] = &["am", "commit", "push"];

impl Rule for GitHookBypass {
    fn name(&self) -> &'static str {
        "git_hook_bypass"
    }

    fn matches(&self, command: &Command) -> bool {
        if !command.text.starts_with("git ") {
            return false;
        }
        HOOKED_COMMANDS.iter().any(|cmd| {
            command.text.split_whitespace().any(|part| part == *cmd)
        })
    }

    fn fix(&self, command: &Command) -> String {
        let hooked_command = HOOKED_COMMANDS
            .iter()
            .find(|cmd| command.text.split_whitespace().any(|part| part == **cmd))
            .unwrap();
        command.text.replace(
            hooked_command,
            &format!("{} --no-verify", hooked_command),
        )
    }
}