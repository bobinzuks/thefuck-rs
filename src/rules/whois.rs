use super::{Command, Rule};

use std::net::ToSocketAddrs;

pub struct Whois;

impl Rule for Whois {
    fn name() -> String {
        "whois".to_string()
    }

    fn matches(cmd: &Command) -> bool {
        // Match any whois command with at least one argument
        if cmd.text.starts_with("whois ") {
            let parts: Vec<&str> = cmd.text.split_whitespace().collect();
            parts.len() >= 2
        } else {
            false
        }
    }

    fn fix(cmd: &Command) -> String {
        let parts: Vec<&str> = cmd.text.split_whitespace().collect();
        let url = parts[1];

        if url.contains('/') {
            // Extract the FQDN (netloc) from the URL
            let netloc = url
                .trim_start_matches("http://")
                .trim_start_matches("https://")
                .split('/')
                .next()
                .unwrap_or(url);
            format!("whois {}", netloc)
        } else if url.contains('.') {
            // Remove the leftmost subdomain
            if let Some(dot_pos) = url.find('.') {
                let without_subdomain = &url[dot_pos + 1..];
                format!("whois {}", without_subdomain)
            } else {
                cmd.text.clone()
            }
        } else {
            cmd.text.clone()
        }
    }
}