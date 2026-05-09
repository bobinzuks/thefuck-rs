use super::{Command, Rule};

pub struct GitPush;

impl Rule for GitPush {
    fn name(&self) -> &str { "git_push" }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.starts_with("git push") &&
        (cmd.output.contains("--set-upstream") ||
         cmd.output.contains("no upstream branch"))
    }

    fn fix(&self, cmd: &Command) -> String {
        let branch = std::process::Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "main".to_string());
        format!("git push --set-upstream origin {}", branch)
    }
}
