use super::{Command, Rule};

pub struct DockerLogin;

impl Rule for DockerLogin {
    fn name(&self) -> &'static str {
        "docker_login"
    }

    fn is_match(&self, command: &Command) -> bool {
        command.script.contains("docker")
            && command.output.contains("access denied")
            && command.output.contains("may require 'docker login'")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        vec![format!("docker login && {}", command.script)]
    }
}
