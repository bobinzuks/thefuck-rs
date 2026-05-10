use super::{Command, Rule};

pub struct WorkonDoesntExists;

impl Rule for WorkonDoesntExists {
    fn name(&self) -> &str { "workon_doesnt_exists" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("workon") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
