use super::{Command, Rule};

pub struct Java;

impl Rule for Java {
    fn name(&self) -> &'static str {
        "java"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.ends_with(".java")
    }

    fn fix(&self, cmd: &Command) -> String {
        cmd.text[..cmd.text.len() - 5].to_string()
    }
}