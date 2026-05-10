use super::{Command, Rule};

pub struct GitMergeUnrelated;

impl Rule for GitMergeUnrelated {
    fn name() -> &'static str {
        "git_merge_unrelated"
    }

    fn matches(cmd: &Command) -> bool {
        cmd.script.contains("merge")
            && cmd.output.contains("fatal: refusing to merge unrelated histories")
    }

    fn fix(cmd: &Command) -> String {
        format!("{} --allow-unrelated-histories", cmd.script)
    }
}