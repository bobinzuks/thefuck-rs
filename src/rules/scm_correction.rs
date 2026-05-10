use super::{Command, Rule};
use std::path::Path;

pub struct ScmCorrection;

static PATH_TO_SCM: &[(&str, &str)] = &[(".git", "git"), (".hg", "hg")];

static WRONG_SCM_PATTERNS: &[(&str, &str)] = &[
    ("git", "fatal: Not a git repository"),
    ("hg", "abort: no repository found"),
];

fn get_actual_scm() -> Option<&'static str> {
    for &(path, scm) in PATH_TO_SCM {
        if Path::new(path).is_dir() {
            return Some(scm);
        }
    }
    None
}

impl Rule for ScmCorrection {
    fn name(&self) -> &'static str {
        "scm_correction"
    }

    fn matches(&self, cmd: &Command) -> bool {
        if cmd.script_parts.is_empty() {
            return false;
        }
        let scm = &cmd.script_parts[0];
        for &(app, pattern) in WRONG_SCM_PATTERNS {
            if app == scm {
                return cmd.output.contains(pattern) && get_actual_scm().is_some();
            }
        }
        false
    }

    fn fix(&self, cmd: &Command) -> String {
        let actual_scm = get_actual_scm().unwrap_or("git");
        let parts: Vec<&str> = cmd.script_parts[1..].iter().map(|s| s.as_str()).collect();
        format!("{} {}", actual_scm, parts.join(" "))
    }
}