use super::{Command, Rule};

pub struct SlLs;

impl Rule for SlLs {
    fn name(&self) -> &'static str {
        "sl_ls"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.trim() == "sl"
    }

    fn fix(&self, _cmd: &Command) -> String {
        "ls".to_string()
    }
}