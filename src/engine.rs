use crate::rules::{get_rules, Command};
use anyhow::Result;
use std::process;

pub fn run() -> Result<()> {
    let cmd = get_last_command()?;
    println!("Last command: {}", cmd.text);

    let rules = get_rules();
    for rule in rules {
        if rule.matches(&cmd) {
            let fixed = rule.fix(&cmd);
            println!("Running: {}", fixed);
            process::Command::new("sh")
                .arg("-c")
                .arg(&fixed)
                .status()?;
            return Ok(());
        }
    }

    println!("No rule matched.");
    Ok(())
}

fn get_last_command() -> Result<Command> {
    let text = std::env::var("THEFUCK_HISTORY")
        .unwrap_or_else(|_| "unknown".to_string());

    // Re-run the failed command, capture stderr as output
    let parts: Vec<&str> = text.splitn(2, ' ').collect();
    let output = if parts.len() >= 1 {
        process::Command::new(parts[0])
            .args(&parts[1..])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stderr).to_string())
            .unwrap_or_default()
    } else {
        String::new()
    };

    Ok(Command {
        text,
        output,
        exit_code: 1,
    })
}
