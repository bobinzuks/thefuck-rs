use super::{Command, Rule};

pub struct GitMainMaster;

impl Rule for GitMainMaster {
    fn name(&self) -> &'static str {
        "git_main_master"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.output.contains("'master'") || cmd.output.contains("'main'")
    }

    fn fix(&self, cmd: &Command) -> String {
        if cmd.output.contains("'master'") {
            cmd.text.replace("master", "main")
        } else {
            cmd.text.replace("main", "master")
        }
    }
}