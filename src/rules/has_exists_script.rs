use super::{Command, Rule};

pub struct HasExistsScript;

impl Rule for HasExistsScript {
    fn name() -> &'static str {
        "has_exists_script"
    }

    fn matches(command: &Command) -> bool {
        if command.text.is_empty() {
            return false;
        }
        let first_word = command.text.split_whitespace().next().unwrap_or("");
        if first_word.is_empty() {
            return false;
        }
        std::path::Path::new(first_word).exists() 
            && command.output.contains("command not found")
    }

    fn fix(command: &Command) -> String {
        format!("./{}", command.text)
    }
}