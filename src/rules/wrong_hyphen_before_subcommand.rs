use super::{Command, Rule};

pub struct WrongHyphenBeforeSubcommand;

impl Rule for WrongHyphenBeforeSubcommand {
    fn name() -> &'static str {
        "wrong_hyphen_before_subcommand"
    }

    fn matches(command: &Command) -> bool {
        let first_part = command.script_parts[0].as_str();
        if !first_part.contains('-') || get_all_executables().contains(first_part) {
            return false;
        }
        let cmd = first_part.split('-').next().unwrap();
        get_all_executables().contains(cmd)
    }

    fn fix(command: &Command) -> String {
        command.text.replacen('-', " ", 1)
    }
}