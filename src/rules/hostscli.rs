use super::{Command, Rule};
use regex::Regex;

pub struct Hostscli;

impl Rule for Hostscli {
    fn name(&self) -> &'static str {
        "hostscli"
    }

    fn matches(&self, command: &Command) -> bool {
        let errors = [
            "Error: No such command",
            "hostscli.errors.WebsiteImportError"
        ];
        errors.iter().any(|&error| command.output.contains(error))
    }

    fn fix(&self, command: &Command) -> String {
        if command.output.contains("hostscli.errors.WebsiteImportError") {
            return "hostscli websites".to_string();
        }

        let re = Regex::new(r#"Error: No such command ".*""#).unwrap();
        if let Some(cap) = re.find(&command.output) {
            let misspelled_command = cap.as_str().to_string();
            let commands = vec![
                "block", "unblock", "websites", "block_all", "unblock_all"
            ];
            replace_command(&command.text, &misspelled_command, &commands)
        } else {
            String::new()
        }
    }
}

fn replace_command(text: &str, misspelled_command: &str, commands: &[&str]) -> String {
    let command_text = misspelled_command
        .trim_start_matches("Error: No such command \"")
        .trim_end_matches('\"');
    
    // Find closest match (simple implementation)
    commands
        .iter()
        .min_by_key(|&&cmd| levenshtein_distance(command_text, cmd))
        .map(|cmd| {
            let args: Vec<&str> = text.split_whitespace().collect();
            if args.len() > 1 {
                format!("{} {}", args[0], cmd)
            } else {
                format!("{} {}", text, cmd)
            }
        })
        .unwrap_or_default()
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            dp[i][j] = std::cmp::min(
                std::cmp::min(dp[i - 1][j] + 1, dp[i][j - 1] + 1),
                dp[i - 1][j - 1] + cost,
            );
        }
    }

    dp[m][n]
}