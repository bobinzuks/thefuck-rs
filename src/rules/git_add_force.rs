use super::{Command, Rule};

pub struct GitAddForce;

impl Rule for GitAddForce {
    fn is_match(&self, command: &Command) -> bool {
        command.script_parts.contains(&"add".to_string())
            && command.output.contains("Use -f if you really want to add them.")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        let new_script = command.script.replace(" add ", " add --force ");
        vec![new_script]
    }
}
