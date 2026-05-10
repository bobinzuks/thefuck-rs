use super::{Command, Rule};

pub struct GitPull;

impl Rule for GitPull {
    fn name() -> &'static str {
        "git_pull"
    }

    fn matches(cmd: &Command) -> bool {
        cmd.text.contains("pull") && cmd.output.contains("set-upstream")
    }

    fn fix(cmd: &Command) -> String {
        let lines: Vec<&str> = cmd.output.split('\n').collect();
        let line = lines[lines.len().saturating_sub(3)].trim();
        let branch = line.split(' ').last().unwrap_or("");
        let set_upstream = line
            .replace("<remote>", "origin")
            .replace("<branch>", branch);
        format!("{} && {}", set_upstream, cmd.text)
    }
}