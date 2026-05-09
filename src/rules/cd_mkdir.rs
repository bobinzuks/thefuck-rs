use super::{Command, Rule};

pub struct CdMkdir;

impl Rule for CdMkdir {
    fn name(&self) -> &str { "cd_mkdir" }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.starts_with("cd ") &&
        cmd.output.contains("No such file or directory")
    }

    fn fix(&self, cmd: &Command) -> String {
        let dir = cmd.text.trim_start_matches("cd ").trim();
        format!("mkdir -p {} && cd {}", dir, dir)
    }
}
