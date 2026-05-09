use std::env;
use std::path::Path;

use super::{Command, Rule};

const MAX_ALLOWED_DIFF: f64 = 0.6;

fn get_sub_dirs(parent: &Path) -> Vec<String> {
    let mut dirs = Vec::new();
    if let Ok(entries) = std::fs::read_dir(parent) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        dirs.push(file_name);
                    }
                }
            }
        }
    }
    dirs
}

fn get_close_matches(target: &str, possibilities: &[String], cutoff: f64) -> Vec<String> {
    let mut matches: Vec<(f64, &String)> = possibilities
        .iter()
        .map(|p| (strsim::jaro_winkler(target, p), p))
        .f