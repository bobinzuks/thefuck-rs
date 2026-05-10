use super::{Command, Rule};

pub struct PathFromHistory;

impl Rule for PathFromHistory {
    fn name(&self) -> &str { "path_from_history" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("path") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
