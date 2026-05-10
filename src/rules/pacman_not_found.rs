use super::{Command, Rule};

pub struct PacmanNotFound;

impl Rule for PacmanNotFound {
    fn name(&self) -> &'static str {
        "pacman_not_found"
    }

    fn matches(&self, command: &Command) -> bool {
        if command.text.is_empty() {
            return false;
        }
        let parts: Vec<&str> = command.text.split_whitespace().collect();
        if parts.is_empty() {
            return false;
        }
        
        let first = parts[0];
        let is_package_manager = matches!(first, "pacman" | "yay" | "pikaur" | "yaourt")
            || (parts.len() > 1 && first == "sudo" && parts[1] == "pacman");
        
        is_package_manager && command.output.contains("error: target not found:")
    }

    fn fix(&self, command: &Command) -> String {
        let parts: Vec<&str> = command.text.split_whitespace().collect();
        let pkg = parts.last().unwrap_or(&"");
        
        // In a real implementation, this would call get_pkgfile equivalent
        // For now return a placeholder
        format!("{} {}", 
            parts[..parts.len()-1].join(" "),
            replace_command(pkg, get_pkgfile(pkg)))
    }
}

// Placeholder functions - actual implementation would need external crate
fn get_pkgfile(_pkg: &str) -> String {
    String::new()
}

fn replace_command(_command: &Command, _old: &str, _new: String) -> String {
    String::new()
}