use super::{Command, Rule};
use regex::Regex;

pub struct Mercurial;

impl Rule for Mercurial {
    fn name(&self) -> &'static str {
        "mercurial"
    }

    fn matches(&self, command: &Command) -> bool {
        let output = &command.output;
        (output.contains("hg: unknown command") && output.contains("(did you mean one of "))
            || (output.contains("hg: command '") && output.contains("' is ambiguous:"))
    }

    fn fix(&self, command: &Command) -> String {
        let possibilities = extract_possibilities(command);
        let mut parts: Vec<&str> = command.text.split(' ').collect();
        
        if let Some(closest) = get_closest(parts[1], &possibilities) {
            parts[1] = closest;
        }
        
        parts.join(" ")
    }
}

fn extract_possibilities(command: &Command) -> Vec<String> {
    let output = &command.output;
    
    // Try first pattern: (did you mean one of ...?)
    let re1 = Regex::new(r"\n\(did you mean one of ([^\?]+)\?\)").unwrap();
    if let Some(caps) = re1.captures(output) {
        return caps[1].split(", ").map(|s| s.to_string()).collect();
    }
    
    // Try second pattern: indented command names
    let re2 = Regex::new(r"\n    ([^$]+)$").unwrap();
    if let Some(caps) = re2.captures(output) {
        return caps[1].split(' ').map(|s| s.to_string()).collect();
    }
    
    Vec::new()
}

fn get_closest<'a>(input: &str, possibilities: &[String]) -> Option<&'a str> {
    // Simple closest match implementation
    let input_lower = input.to_lowercase();
    
    possibilities
        .iter()
        .min_by_key(|p| {
            let p_lower = p.to_lowercase();
            levenshtein_distance(&input_lower, &p_lower)
        })
        .map(|s| s.as_str())
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();
    
    let mut matrix = vec![vec![0usize; b_len + 1]; a_len + 1];
    
    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    for j in 0..=b_len {
        matrix[0][j] = j;
    }
    
    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i-1] == b_chars[j-1] { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(matrix[i-1][j] + 1, matrix[i][j-1] + 1),
                matrix[i-1][j-1] + cost
            );
        }
    }
    
    matrix[a_len][b_len]
}