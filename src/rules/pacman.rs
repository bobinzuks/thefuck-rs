use super::{Command, Rule};
use std::process::Command as StdCommand;

pub struct Pacman;

impl Rule for Pacman {
    fn name(&self) -> &'static str {
        "pacman"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.output.contains("not found") && get_pkgfile(&cmd.text)
    }

    fn fix(&self, cmd: &Command) -> String {
        let packages = get_pkgfile_packages(&cmd.text);
        let pacman = get_pacman();
        
        packages.iter()
            .map(|package| format!("{} -S {} && {}", pacman, package, cmd.text))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn get_pkgfile(command: &str) -> bool {
    let output = StdCommand::new("pkgfile")
        .args(["-b", "-v", command])
        .output();
    
    match output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}

fn get_pkgfile_packages(command: &str) -> Vec<String> {
    let output = StdCommand::new("pkgfile")
        .args(["-b", "-v", command])
        .output();
    
    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout.lines()
                .filter_map(|line| {
                    let parts: Vec<&str> = line.splitn(2, '/').collect();
                    if parts.len() == 2 {
                        Some(parts[0].to_string())
                    } else {
                        None
                    }
                })
                .collect()
        }
        Err(_) => vec![],
    }
}

fn get_pacman() -> String {
    // Check for yay, paru, or fallback to pacman
    for pacman_cmd in &["yay", "paru", "pacman"] {
        if StdCommand::new(pacman_cmd)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return pacman_cmd.to_string();
        }
    }
    "pacman".to_string()
}