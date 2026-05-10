use super::{Command, Rule};

pub struct Tmux;

impl Rule for Tmux {
    fn name(&self) -> &str { "tmux" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("tmux") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
