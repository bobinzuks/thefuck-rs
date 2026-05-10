use super::{Command, Rule};

pub struct GitHelpAliased;

impl Rule for GitHelpAliased {
    fn name() -> &'static str {
        "git_help_aliased"
    }

    fn matches(cmd: &Command) -> bool {
        cmd.text.contains("help") && cmd.output.contains(" is aliased to ")
    }

    fn fix(cmd: &Command) -> String {
        let aliased = cmd.output
            .split('`')
            .nth(2)
            .and_then(|s| s.split('\'')
                .next())
            .and_then(|s| s.split(' ')
                .next())
            .unwrap_or("");
        
        format!("git help {}", aliased)
    }
}