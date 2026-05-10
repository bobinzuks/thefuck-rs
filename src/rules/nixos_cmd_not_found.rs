use super::{Command, Rule};
use regex::Regex;

pub struct NixosCmdNotFound;

impl Rule for NixosCmdNotFound {
    fn name(&self) -> &'static str {
        "nixos_cmd_not_found"
    }

    fn matches(&self, command: &Command) -> bool {
        let re = Regex::new(r"nix-env -iA ([^\s]*)").unwrap();
        re.is_match(&command.output)
    }

    fn fix(&self, command: &Command) -> String {
        let re = Regex::new(r"nix-env -iA ([^\s]*)").unwrap();
        let caps = re.captures(&command.output).unwrap();
        let name = caps.get(1).unwrap().as_str();
        format!("nix-env -iA {} && {}", name, command.text)
    }
}