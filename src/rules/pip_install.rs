pub struct PipInstall;

impl Rule for PipInstall {
    fn name(&self) -> &'static str {
        "pip_install"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.contains("pip install")
            && cmd.output.contains("Permission denied")
    }

    fn fix(&self, cmd: &Command) -> String {
        if !cmd.text.contains("--user") {
            cmd.text.replace(" install ", " install --user ")
        } else {
            format!("sudo {}", cmd.text.replace(" --user", ""))
        }
    }
}