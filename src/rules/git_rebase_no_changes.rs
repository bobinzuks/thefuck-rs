use super::{Command, Rule};

pub struct GitRebaseNoChanges;

impl Rule for GitRebaseNoChanges {
    fn name(&self) -> &'static str {
        "git_rebase_no_changes"
    }

    fn matches(&self, command: &Command) -> bool {
        let script_parts: Vec<&str> = command.text.split_whitespace().collect();
        let has_rebase = script_parts.contains(&"rebase");
        let has_continue = script_parts.contains(&"--continue");
        
        has_rebase && has_continue && 
        command.output.contains("No changes - did you forget to use 'git add'?")
    }

    fn fix(&self, _command: &Command) -> String {
        "git rebase --skip".to_string()
    }
}