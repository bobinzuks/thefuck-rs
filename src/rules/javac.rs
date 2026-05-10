use super::{Command, Rule};

pub struct Javac;

impl Rule for Javac {
    fn name(&self) -> &'static str {
        "javac"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text == "javac" && !cmd.output.ends_with(".java")
    }

    fn fix(&self, cmd: &Command) -> String {
        format!("{}.java", cmd.text)
    }
}