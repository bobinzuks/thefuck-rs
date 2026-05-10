use super::{Command, Rule};

pub struct GoRun;

impl Rule for GoRun {
    fn name() -> &'static str {
        "go_run"
    }

    fn matches(cmd: &Command) -> bool {
        cmd.text.starts_with("go run ")
            && !cmd.text.ends_with(".go")
    }

    fn fix(cmd: &Command) -> String {
        format!("{}.go", cmd.text)
    }
}