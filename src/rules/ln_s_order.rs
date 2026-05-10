use super::{Command, Rule};

pub struct LnSOrder;

impl Rule for LnSOrder {
    fn name(&self) -> &'static str {
        "ln_s_order"
    }

    fn matches(&self, cmd: &Command) -> bool {
        let script_parts: Vec<&str> = cmd.text.split_whitespace().collect();
        
        if script_parts.is_empty() || script_parts[0] != "ln" {
            return false;
        }
        
        let has_symbolic = script_parts.iter().any(|&p| p == "-s" || p == "--symbolic");
        if !has_symbolic {
            return false;
        }
        
        if !cmd.output.contains("File exists") {
            return false;
        }
        
        get_destination(&script_parts).is_some()
    }

    fn fix(&self, cmd: &Command) -> String {
        let script_parts: Vec<&str> = cmd.text.split_whitespace().collect();
        let destination = get_destination(&script_parts).unwrap();
        
        let mut parts: Vec<&str> = script_parts.clone();
        parts.retain(|&p| p != destination);
        parts.push(destination);
        parts.join(" ")
    }
}

fn get_destination(script_parts: &[&str]) -> Option<&str> {
    for &part in script_parts {
        if part != "ln" && part != "-s" && part != "--symbolic" && std::path::Path::new(part).exists() {
            return Some(part);
        }
    }
    None
}