use super::{Command, Rule};

use regex::Regex;
use once_cell::sync::Lazy;

static WARNING_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"Warning: (?:.(?!is ))+ is already installed and up-to-date")
        .unwrap()
});

static MESSAGE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"To reinstall (?:(?!, ).)+, run `brew reinstall [^`]+`")
        .unwrap()
});

pub struct BrewReinstall;

impl Rule for BrewReinstall {
    fn name(&self) -> &'static str {
        "brew_reinstall"
    }

    fn match_command(&self, command: &Command) -> bool {
        command.script.contains("install")
            && WARNING_REGEX.is_match(&command.output)
            && MESSAGE_REGEX.is_match(&command.output)
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        vec![comm