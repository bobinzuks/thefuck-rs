use super::{Command, Rule};

pub struct PacmanNotFound;

impl Rule for PacmanNotFound {
    fn name(&self) -> &str { "pacman_not_found" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("pacman") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
