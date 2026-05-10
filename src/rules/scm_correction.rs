use super::{Command, Rule};

pub struct ScmCorrection;

impl Rule for ScmCorrection {
    fn name(&self) -> &str { "scm_correction" }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.output.contains("unknown command") ||
        cmd.output.contains("is not a command")
    }

    fn fix(&self, cmd: &Command) -> String {
        cmd.text.clone()
    }
}
