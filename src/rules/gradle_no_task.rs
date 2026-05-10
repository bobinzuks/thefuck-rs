use super::{Command, Rule};

pub struct GradleNoTask;

impl Rule for GradleNoTask {
    fn name(&self) -> &str { "gradle_no_task" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("gradle") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
