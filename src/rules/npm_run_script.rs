use super::{Command, Rule};

pub struct NpmRunScript;

impl Rule for NpmRunScript {
    fn name(&self) -> &'static str {
        "npm_run_script"
    }

    fn matches(&self, command: &Command) -> bool {
        let script_parts: Vec<&str> = command.text.split_whitespace().collect();
        
        // Check if it's an npm command
        if script_parts.first() != Some(&"npm") {
            return false;
        }

        // Check for "Usage: npm <command>" in output
        if !command.output.contains("Usage: npm <command>") {
            return false;
        }

        // Check that no part starts with "ru"
        if script_parts.iter().any(|part| part.starts_with("ru")) {
            return false;
        }

        // Check that the second part is a known script
        if script_parts.len() < 2 {
            return false;
        }

        let scripts = get_scripts();
        scripts.contains(script_parts[1])
    }

    fn fix(&self, command: &Command) -> String {
        let mut parts: Vec<&str> = command.text.split_whitespace().collect();
        parts.insert(1, "run-script");
        parts.join(" ")
    }
}

fn get_scripts() -> Vec<String> {
    // Implementation would parse package.json scripts
    // For now return empty vector as placeholder
    Vec::new()
}