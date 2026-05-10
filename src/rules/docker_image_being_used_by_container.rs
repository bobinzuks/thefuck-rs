use super::{Command, Rule};

pub struct DockerImageBeingUsedByContainer;

impl Rule for DockerImageBeingUsedByContainer {
    fn name(&self) -> &str { "docker_image_being_used_by_container" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("docker") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
