pub struct GitPullClone;

impl Rule for GitPullClone {
    fn name(&self) -> &'static str {
        "git_pull_clone"
    }

    fn matches(&self, command: &Command) -> bool {
        command.text.starts_with("git pull")
            && command.output.contains("fatal: Not a git repository")
            && command.output.contains("Stopping at filesystem boundary (GIT_DISCOVERY_ACROSS_FILESYSTEM not set).")
    }

    fn fix(&self, command: &Command) -> String {
        command.text.replace("pull", "clone")
    }
}