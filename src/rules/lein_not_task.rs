use super::{Command, Rule};
use regex::Regex;

pub struct LeinNotTask;

impl Rule for LeinNotTask {
    fn name(&self) -> &'static str {
        "lein_not_task"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.starts_with("lein")
            && cmd.output.contains("is not a task. See 'lein help'")
            && cmd.output.contains("Did you mean this?")
    }

    fn fix(&self, cmd: &Command) -> String {
        let re_broken = Regex::new(r"'([^']*)' is not a task").unwrap();
        let broken_cmd = re_broken
            .captures(&cmd.output)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");

        let re_help = Regex::new(r"'([^']*)'").unwrap();
        let new_cmds: Vec<&str> = cmd
            .output
            .split('\n')
            .filter(|line| line.contains("Did you mean this?"))
            .flat_map(|line| re_help.find_iter(line))
            .map(|m| m.as_str().trim_matches('\''))
            .collect();

        if new_cmds.is_empty() {
            return cmd.text.clone();
        }

        // Replace the broken command with the first suggestion
        cmd.text.replacen(broken_cmd, new_cmds[0], 1)
    }
}