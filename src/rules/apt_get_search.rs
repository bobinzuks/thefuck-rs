use super::{Command, Rule};

pub struct AptGetSearch;

impl Rule for AptGetSearch {
    fn name(&self) -> &str { "apt_get_search" }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.contains("apt")
    }

    fn fix(&self, cmd: &Command) -> String {
        cmd.text.clone()
    }
}
