use super::{Command, Rule};

pub struct MissingSpaceBeforeSubcommand;

impl Rule for MissingSpaceBeforeSubcommand {
    fn name() -> &'static str {
        "missing_space_before_subcommand"
    }

    fn matches(cmd: &Command) -> bool {
        let script_parts: Vec<&str> = cmd.text.split_whitespace().collect();
        if script_parts.is_empty() {
            return false;
        }
        let first_part = script_parts[0];
        
        if get_all_executables().contains(&first_part.to_string()) {
            return false;
        }
        
        get_executable(first_part).is_some()
    }

    fn fix(cmd: &Command) -> String {
        let script_parts: Vec<&str> = cmd.text.split_whitespace().collect();
        let first_part = script_parts[0];
        
        if let Some(executable) = get_executable(first_part) {
            let mut new_text = cmd.text.clone();
            if let Some(pos) = new_text.find(&executable) {
                new_text.insert_str(pos + executable.len(), " ");
                return new_text;
            }
        }
        cmd.text.clone()
    }
}

fn get_all_executables() -> Vec<String> {
    // This would need to be implemented based on the actual system
    // For now, returning an empty vec as placeholder
    Vec::new()
}

fn get_executable(script_part: &str) -> Option<String> {
    for executable in get_all_executables() {
        if executable.len() > 1 && script_part.starts_with(&executable) {
            return Some(executable);
        }
    }
    None
}