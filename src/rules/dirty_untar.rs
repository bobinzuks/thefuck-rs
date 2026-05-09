use super::{Command, Rule};
use std::path::Path;

const TAR_EXTENSIONS: &[&str] = &[
    ".tar", ".tar.Z", ".tar.bz2", ".tar.gz", ".tar.lz", ".tar.lzma", ".tar.xz",
    ".taz", ".tb2", ".tbz", ".tbz2", ".tgz", ".tlz", ".txz", ".tz",
];

fn is_tar_extract(cmd: &str) -> bool {
    if cmd.contains("--extract") {
        return true;
    }

    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.len() > 1 {
        let flags = parts[1];
        if flags.contains('x') {
            return true;
        }
    }
    false
}

fn tar_file(cmd_parts: &[String]) -> Option<(String, String)> {
    for c in cmd_parts {
        for ext in TAR_EXTENSIONS {
            if c.ends_with(ext) {
                let stem = c[..c.len() - ext.len()].to_string();
                return Some((c.clo