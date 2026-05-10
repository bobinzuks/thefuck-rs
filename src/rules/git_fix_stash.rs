use super::{Command, Rule};

pub struct GitFixStash;

impl Rule for GitFixStash {
    fn name(&self) -> &'static str {
        "git_fix_stash"
    }

    fn matches(&self, cmd: &Command) -> bool {
        if cmd.script_parts.len() > 1 && cmd.script_parts[1] == "stash" {
            if let Some(output) = &cmd.output {
                return output.contains("usage:");
            }
        }
        false
    }

    fn fix(&self, cmd: &Command) -> String {
        const STASH_COMMANDS: &[&str] = &[
            "apply",
            "branch",
            "clear",
            "drop",
            "list",
            "pop",
            "save",
            "show",
        ];

        let stash_cmd = &cmd.script_parts[2];
        let fixed = get_closest(stash_cmd, STASH_COMMANDS, false);

        if let Some(fixed_cmd) = fixed {
            replace_argument(&cmd.script, stash_cmd, fixed_cmd)
        } else {
            let mut new_parts = cmd.script_parts.clone();
            new_parts.insert(2, "save".to_string());
            new_parts.join(" ")
        }
    }
}

fn get_closest<'a>(input: &str, options: &[&'a str], _fallback_to_first: bool) -> Option<&'a str> {
    let input_lower = input.to_lowercase();
    let mut best_match: Option<&str> = None;
    let mut best_distance = usize::MAX;

    for &option in options {
        let option_lower = option.to_lowercase();
        let distance = levenshtein_distance(&input_lower, &option_lower);
        if distance < best_distance {
            best_distance = distance;
            best_match = Some(option);
        }
    }

    if best_distance <= 2 || (best_distance as f64) <= (input.len() as f64 * 0.5) {
        best_match
    } else {
        None
    }
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
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[a_len][b_len]
}

fn replace_argument(script: &str, old: &str, new: &str) -> String {
    script.replacen(old, new, 1)
}