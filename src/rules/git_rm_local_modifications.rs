use super::{Command, Rule};

pub struct GitRmLocalModifications;

impl Rule for GitRmLocalModifications {
    fn name(&self) -> &'static str {
        "git_rm_local_modifications"
    }

    fn matches(&self, command: &Command) -> bool {
        command.text.contains(" rm ")
            && command.output.contains("error: the following file has local modifications")
            && command.output.contains("use --cached to keep the file, or -f to force removal")
    }

    fn fix(&self, command: &Command) -> Option<String> {
        let mut parts: Vec<&str> = command.text.split_whitespace().collect();
        if let Some(rm_index) = parts.iter().position(|&s| s == "rm") {
            let insert_index = rm_index + 1;
            parts.insert(insert_index, "--cached");
            let first = parts.join(" ");
            
            parts[insert_index] = "-f";
            let second = parts.join(" ");
            
            Some(format!("{}\n{}", first, second))
        } else {
            None
        }
    }
}