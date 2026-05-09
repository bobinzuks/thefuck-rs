use super::{Command, Rule};

pub struct AptListUpgradable;

impl Rule for AptListUpgradable {
    fn is_match(&self, command: &Command) -> bool {
        command.output.contains("apt list --upgradable")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        vec!["apt list --upgradable".to_string()]
    }

    fn is_available(&self) -> bool {
        apt_available()
    }
}

fn apt_available() -> bool {
    // Check if apt is available on the system
    std::process::Command::new("which")
        .arg("apt")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
