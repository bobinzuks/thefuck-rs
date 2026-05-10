use super::{Command, Rule};
use std::process::Command as StdCommand;

pub struct IfconfigDeviceNotFound;

impl Rule for IfconfigDeviceNotFound {
    fn name(&self) -> &'static str {
        "ifconfig_device_not_found"
    }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.text.starts_with("ifconfig") 
            && cmd.output.contains("error fetching interface information: Device not found")
    }

    fn fix(&self, cmd: &Command) -> Option<String> {
        let interface = cmd.output.split_whitespace().next()?.trim_end_matches(':').to_string();
        let possible = _get_possible_interfaces();
        
        // Find best match using simple string similarity
        let best_match = possible.iter()
            .fold(None, |best: Option<&String>, candidate| {
                let dist = levenshtein_distance(&interface, candidate);
                match best {
                    None => Some((candidate, dist)),
                    Some((_, best_dist)) if dist < best_dist => Some((candidate, dist)),
                    _ => best
                }
            });
        
        best_match.map(|(iface, _)| {
            cmd.text.replacen(&interface, iface, 1)
        })
    }
}

fn _get_possible_interfaces() -> Vec<String> {
    let output = StdCommand::new("ifconfig")
        .arg("-a")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();

    output.lines()
        .filter(|line| !line.is_empty() && !line.starts_with(' '))
        .filter_map(|line| line.split_whitespace().next())
        .map(|s| s.to_string())
        .collect()
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let mut result = 0;
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    
    if a_chars.len() < b_chars.len() {
        return levenshtein_distance(b, a);
    }
    
    let mut prev_row: Vec<usize> = (0..=b_chars.len()).collect();
    
    for (i, ca) in a_chars.iter().enumerate() {
        let mut new_row = vec![i + 1];
        for (j, cb) in b_chars.iter().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            let val = std::cmp::min(
                new_row[j] + 1,
                std::cmp::min(
                    prev_row[j + 1] + 1,
                    prev_row[j] + cost
                )
            );
            new_row.push(val);
        }
        prev_row = new_row;
    }
    
    prev_row[b_chars.len()]
}