use super::{Command, Rule};

pub struct ManNoSpace;

impl Rule for ManNoSpace {
    fn name(&self) -> &'static str {
        "man_no_space"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.starts_with("man")
            && cmd.output.to_lowercase().contains("command not found")
    }

    fn fix(&self, cmd: &Command) -> String {
        format!("man {}", &cmd.text[3..])
    }
}