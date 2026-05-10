use super::{Command, Rule};

pub struct GitCloneMissing;

impl Rule for GitCloneMissing {
    fn name(&self) -> &'static str {
        "git_clone_missing"
    }

    fn matches(&self, command: &Command) -> bool {
        // We want it to be a URL by itself
        if command.script_parts.len() != 1 {
            return false;
        }
        
        let part = &command.script_parts[0];
        
        // Check if the command exists (if it does, it's not a URL)
        if which(part).is_some() {
            return false;
        }
        
        // Ensure we got the error we expected
        let output = command.output.to_lowercase();
        if !(output.contains("no such file or directory")
            || output.contains("not found")
            || output.contains("is not recognised as"))
        {
            return false;
        }
        
        // Parse the URL
        let url = urlparse(&command.script, "ssh");
        
        // HTTP URLs need a network address
        if !url.netloc.is_empty() && url.scheme != "ssh" {
            return true;
        }
        
        // SSH needs a username and a splitter between the path
        if url.scheme == "ssh" {
            if command.script.contains('@') && command.script.contains(':') {
                return true;
            }
            return false;
        }
        
        // Check scheme
        matches!(url.scheme.as_str(), "http" | "https" | "ssh")
    }

    fn fix(&self, command: &Command) -> String {
        format!("git clone {}", command.script)
    }
}

// Helper struct for URL parsing
struct Url {
    scheme: String,
    netloc: String,
}

// Simple URL parser
fn urlparse(url: &str, default_scheme: &str) -> Url {
    let url = url.trim();
    
    // Check if URL has a scheme
    let (scheme, rest) = if let Some(pos) = url.find("://") {
        (url[..pos].to_lowercase(), &url[pos + 3..])
    } else {
        (default_scheme.to_string(), url)
    };
    
    // Extract netloc (host:port or user@host)
    let netloc = if let Some(pos) = rest.find('/') {
        rest[..pos].to_string()
    } else {
        rest.to_string()
    };
    
    Url { scheme, netloc }
}

// Simple which implementation
fn which(cmd: &str) -> Option<String> {
    use std::process::Command;
    Command::new("which")
        .arg(cmd)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| {
            String::from_utf8(o.stdout)
                .ok()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        })
}