use super::{Command, Rule};

pub struct GitTagForce;

impl Rule for GitTagForce {
    fn name(&self) -> &'static str {
        "git_tag_force"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.contains("tag")
            && cmd.text.split_whitespace().any(|part| part == "tag")
            && cmd.output.contains("already exists")
    }

    fn fix(&self, cmd: &Command) -> String {
        cmd.text.replace(" tag ", " tag --force ")
            .replace("tag ", "tag --force ")
            .replace(" tag", " tag --force")
    }
}