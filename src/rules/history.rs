use super::{Command, Rule};

pub struct History;

impl Rule for History {
    fn name(&self) -> &'static str {
        "history"
    }

    fn matches(&self, cmd: &Command) -> bool {
        !get_close_matches(&cmd.text, &get_valid_history_without_current(cmd)).is_empty()
    }

    fn fix(&self, cmd: &Command) -> Option<String> {
        get_closest(&cmd.text, &get_valid_history_without_current(cmd))
    }
}

fn get_close_matches(script: &str, history: &[String]) -> Vec<String> {
    // Simple implementation: return strings that have a common prefix
    let min_similarity = 0.6;
    history
        .iter()
        .filter(|h| {
            let distance = levenshtein_distance(script, h);
            1.0 - (distance as f64 / script.len().max(h.len()) as f64) >= min_similarity
        })
        .cloned()
        .collect()
}

fn get_closest(script: &str, history: &[String]) -> Option<String> {
    history
        .iter()
        .min_by(|a, b| {
            let dist_a = levenshtein_distance(script, a);
            let dist_b = levenshtein_distance(script, b);
            dist_a.cmp(&dist_b)
        })
        .cloned()
}

fn get_valid_history_without_current(cmd: &Command) -> Vec<String> {
    // This is a placeholder - actual implementation would depend on how history is accessed
    // For now, return an empty vec to demonstrate the structure
    Vec::new()
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();
    
    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];
    
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
                matrix[i-1][j] + 1,
                std::cmp::min(
                    matrix[i][j-1] + 1,
                    matrix[i-1][j-1] + cost
                )
            );
        }
    }
    
    matrix[a_len][b_len]
}