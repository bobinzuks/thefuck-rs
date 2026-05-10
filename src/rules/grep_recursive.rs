use super::{Command, Rule};

pub struct GrepRecursive;

impl Rule for GrepRecursive {
    fn name() -> &'static str {
        "grep_recursive"
    }

    fn matches(cmd: &Command) -> bool {
        cmd.output.to_lowercase().contains("is a directory")
    }

    fn fix(cmd: &Command) -> String {
        format!("grep -r {}", &cmd.text[5..])
    }
}