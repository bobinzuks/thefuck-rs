use super::{Command, Rule};

pub struct SshKnownHosts;

impl Rule for SshKnownHosts {
    fn name(&self) -> &str { "ssh_known_hosts" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("ssh") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
