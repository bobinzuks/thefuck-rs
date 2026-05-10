use super::{Command, Rule};

pub struct GitCommitAdd;

impl Rule for GitCommitAdd {
    fn name(&self) -> &'static str {
        "git_commit_add"
    }

    fn matches(&self, command: &Command) -> bool {
        command.text.contains("commit")
            && command.output.contains("no changes added to commit")
    }

    fn fix(&self, command: &Command) -> String {
        let script = &command.text;
        for opt in &["-a", "-p"] {
            if let Some(pos) = script.find("commit") {
                let result = format!(
                    "{}{} {}",
                    &script[..pos],
                    "commit",
                    opt,
                    &script[pos + 6..]
                );
                return result;
            }
        }
        script.clone()
    }
}