use super::{Command, Rule};
use regex::Regex;

pub struct GitFlagAfterFilename;

impl Rule for GitFlagAfterFilename {
    fn name(&self) -> &'static str {
        "git_flag_after_filename"
    }

    fn matches(&self, cmd: &Command) -> bool {
        let error_pattern = Regex::new(r"fatal: bad flag '(.*?)' used after filename").unwrap();
        let error_pattern2 = Regex::new(r"fatal: option '(.*?)' must come before non-option arguments").unwrap();
        
        error_pattern.is_match(&cmd.output) || error_pattern2.is_match(&cmd.output)
    }

    fn fix(&self, cmd: &Command) -> String {
        let error_pattern = Regex::new(r"fatal: bad flag '(.*?)' used after filename").unwrap();
        let error_pattern2 = Regex::new(r"fatal: option '(.*?)' must come before non-option arguments").unwrap();
        
        let bad_flag = if let Some(caps) = error_pattern.captures(&cmd.output) {
            caps.get(1).unwrap().as_str().to_string()
        } else if let Some(caps) = error_pattern2.captures(&cmd.output) {
            caps.get(1).unwrap().as_str().to_string()
        } else {
            return String::new();
        };

        let mut command_parts: Vec<String> = cmd.script_parts.clone();
        
        let bad_flag_index = command_parts.iter().position(|p| p == &bad_flag).unwrap();
        
        let mut filename_index = 0;
        for index in (0..bad_flag_index).rev() {
            if !command_parts[index].starts_with('-') {
                filename_index = index;
                break;
            }
        }

        command_parts.swap(bad_flag_index, filename_index);
        command_parts.join(" ")
    }
}