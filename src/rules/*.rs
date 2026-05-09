use super::{Command, Rule};

pub struct CargoLockCargoToml;

impl Rule for CargoLockCargoToml {
    fn name(&self) -> &str {
        "cargo_lock_cargo_toml"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.starts_with("cargo lock")
            || cmd.text.starts_with("cargo toml")
            || cmd.text.starts_with("cargo convert")
            || cmd.text.starts_with("cargo rule")
            || cmd.text.starts_with("cargo readme")
            || cmd.text.starts_with("cargo src")
            || cmd.text.starts_with("cargo target")
    }

    fn fix(&self, cmd: &Command) -> String {
        // Replace common mistyped cargo subcommands with the correct ones
        let text = cmd.text.clone();
        text.replace("cargo lock", "cargo update")
            .replace("cargo toml", "cargo metadata")
            .replace("cargo convert", "cargo build")
            .replace("cargo rule", "cargo check")
            .replace("cargo readme", "cargo doc")
            .replace("cargo src", "cargo build")
            .replace("cargo target", "cargo build")
    }

    fn priority(&self) -> u8 {
        50
    }
}
