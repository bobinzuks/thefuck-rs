use super::{Command, Rule};

pub struct GitBisectUsage;

impl Rule for GitBisectUsage {
    fn name(&self) -> &str { "git_bisect_usage" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
