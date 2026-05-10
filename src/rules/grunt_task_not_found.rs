use super::{Command, Rule};

pub struct GruntTaskNotFound;

impl Rule for GruntTaskNotFound {
    fn name(&self) -> &str { "grunt_task_not_found" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("grunt") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
