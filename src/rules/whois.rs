use super::{Command, Rule};

pub struct Whois;

impl Rule for Whois {
    fn name(&self) -> &str { "whois" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("whois") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
