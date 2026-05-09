use super::{Command, Rule};
use std::fs;
use std::path::Path;
use std::process::Command as ProcessCommand;

fn is_bad_zip(file: &str) -> bool {
    let path = Path::new(file);
    if !path.exists() {
        return false;
    }
    match fs::read(path) {
        Ok(data) => {
            // Check if it's a valid zip with more than one file
            use std::io::Cursor;
            match zip::ZipArchive::new(Cursor::new(data)) {
                Ok(mut archive) => archive.len() > 1,
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}

fn zip_file(command: &Command) -> Option<String> {
    for arg in &command.script_parts[1..] {
        if !arg.starts_with('-') {
            if arg.ends_with(".zip") {
                return Some(arg.clone());
         