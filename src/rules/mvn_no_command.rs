use super::{Command, Rule};

pub struct MvnNoCommand;

impl Rule for MvnNoCommand {
    fn name(&self) -> &'static str {
        "mvn_no_command"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.output.contains("No goals have been specified for this build")
    }

    fn fix(&self, cmd: &Command) -> String {
        format!("{}\n{}", cmd.script.clone() + " clean package", cmd.script.clone() + " clean install")
    }
}