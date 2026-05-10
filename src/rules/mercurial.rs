use super::{Command, Rule};

pub struct Mercurial;

impl Rule for Mercurial {
    fn name(&self) -> &str { "mercurial" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("mercurial") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
