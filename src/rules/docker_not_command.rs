use super::{Command, Rule};

pub struct DockerNotCommand;

impl Rule for DockerNotCommand {
    fn name(&self) -> &str { "docker_not_command" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("docker") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
