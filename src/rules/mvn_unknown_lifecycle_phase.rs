use super::{Command, Rule};

pub struct MvnUnknownLifecyclePhase;

impl Rule for MvnUnknownLifecyclePhase {
    fn name(&self) -> &str { "mvn_unknown_lifecycle_phase" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("mvn") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
