use super::{Command, Rule};

pub struct DockerRemoveContainerFirst;

impl Rule for DockerRemoveContainerFirst {
    fn name(&self) -> &'static str {
        "docker_remove_container_first"
    }

    fn description(&self) -> &'static str {
        "Prepend docker container rm -f to docker image rm when container is using the image"
    }

    fn is_match(&self, command: &Command) -> bool {
        command.output.contains("image is being used by running container")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        let container_id = command.output
            .trim()
            .split(' ')
            .last()
            .unwrap_or("")
            .to_string();
        
        vec![format!(
            "docker container rm -f {} && {}",
            container