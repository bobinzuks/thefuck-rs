use super::{Command, Rule};

pub struct GradleWrapper;

impl Rule for GradleWrapper {
    fn name(&self) -> &'static str {
        "gradle_wrapper"
    }

    fn matches(&self, cmd: &Command) -> bool {
        if cmd.text.is_empty() {
            return false;
        }
        
        let first_arg = cmd.text.split_whitespace().next().unwrap_or("");
        let gradle_not_found = !which(first_arg) && cmd.output.contains("not found");
        
        gradle_not_found && std::path::Path::new("gradlew").exists()
    }

    fn fix(&self, cmd: &Command) -> String {
        let args: Vec<&str> = cmd.text.split_whitespace().skip(1).collect();
        format!("./gradlew {}", args.join(" "))
    }
}

fn which(program: &str) -> bool {
    std::env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .any(|dir| std::path::Path::new(dir).join(program).exists())
}