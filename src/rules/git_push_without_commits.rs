use super::{Command, Rule};

pub struct GitPush;

impl Rule for GitPush {
    fn name(&self) -> &str {
        "git_push"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.starts_with("git push")
            && (cmd.output.contains("fatal: The current branch")
                || cmd.output.contains("fatal: No configured push destination")
                || cmd.output.contains("fatal: No upstream branch"))
    }

    fn fix(&self, cmd: &Command) -> String {
        // Extract the branch name from the error message
        let branch = cmd.output
            .lines()
            .find(|line| line.contains("'") && line.contains("'"))
            .and_then(|line| {
                let start = line.find("'")? + 1;
                let end = line[start..].find("'")?;
       