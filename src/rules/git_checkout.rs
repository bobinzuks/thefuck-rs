use super::{Command, Rule};

pub struct GitCheckout;

impl Rule for GitCheckout {
    fn name(&self) -> &str { "git_checkout" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
