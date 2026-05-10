use super::{Command, Rule};

pub struct RmRoot;

impl Rule for RmRoot {
    fn name(&self) -> &'static str {
        "rm_root"
    }

    fn matches(&self, cmd: &Command) -> bool {
        !cmd.text.is_empty()
            && cmd.text.split_whitespace().collect::<Vec<&str>>().contains(&"rm")
            && cmd.text.split_whitespace().collect::<Vec<&str>>().contains(&"/")
            && !cmd.text.contains("--no-preserve-root")
            && cmd.output.contains("--no-preserve-root")
    }

    fn fix(&self, cmd: &Command) -> String {
        format!("{} --no-preserve-root", cmd.text)
    }
}