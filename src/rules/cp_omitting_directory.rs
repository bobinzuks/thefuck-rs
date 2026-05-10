use super::{Command, Rule};

pub struct CpOmittingDirectory;

impl Rule for CpOmittingDirectory {
    fn name(&self) -> &str { "cp_omitting_directory" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("cp") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
