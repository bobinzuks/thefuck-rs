use super::{Command, Rule};

pub struct DockerLogin;

impl Rule for DockerLogin {
    fn name(&self) -> &str { "docker_login" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("docker") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
