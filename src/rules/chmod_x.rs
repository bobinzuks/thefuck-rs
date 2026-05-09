use super::{Command, Rule};
use std::path::Path;

pub struct ChmodX;

impl Rule for ChmodX {
    fn is_match(&self, command: &Command) -> bool {
        command.script.starts_with("./")
            && command.output.to_lowercase().contains("permission denied")
            && Path::new(&command.script_parts[0]).exists()
            && !is_executable(&command.script_parts[0])
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        let script = &command.script;
        let filename = &command.script_parts[0][2..]; // Remove "./" prefix
        vec![format!("chmod +x {} && {}", filename, script)]
    }
}

fn is_executable(path: &str) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(path)
            .map(|m| m.pe