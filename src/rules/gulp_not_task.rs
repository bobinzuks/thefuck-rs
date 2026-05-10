use super::{Command, Rule};

pub struct GulpNotTask;

impl Rule for GulpNotTask {
    fn name(&self) -> &str { "gulp_not_task" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("gulp") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
