use super::{Command, Rule};
use std::path::Path;

pub struct CatToLs;

impl Rule for CatToLs {
    fn name(&self) -> &'static str {
        "cat_to_ls"
    }

    fn match_command(&self, command: &Command) -> Option<Box<dyn Rule>> {
        if command.script_parts.len() < 2 {
            return None;
        }

        let output_starts_with_cat = command.output.as_deref()
            .map(|o| o.starts_with("cat: "))
            .unwrap_or(false);

        let target_path = &command.script_parts[1];
        let is_directory = Path::new(target_path).is_dir();

        if output_starts_with_cat && is_directory {
            Some(Box::new(CatToLs))
        } else {
            None
        }
    }

    fn get_new_command(&self, command: &Command) -> Option<String> {
        Some(command.scrip