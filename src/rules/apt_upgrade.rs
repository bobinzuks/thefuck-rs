use super::{Command, Rule};

pub struct AptListUpgradable;

impl Rule for AptListUpgradable {
    fn is_match(&self, command: &Command) -> bool {
        command.script == "apt list --upgradable" 
            && command.output.trim().lines().count() > 1
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        vec!["apt upgrade".to_string()]
    }
}

pub fn apt_list_upgradable() -> Box<dyn Rule> {
    Box::new(AptListUpgradable)
}
