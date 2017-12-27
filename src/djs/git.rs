
use std::process::{Command};

pub fn guess_branch() -> Option<String> {
    debug!("guess_branch");

    let result = Command::new("git")
            .args(&["symbolic-ref", "head"])
            .output();

    match result {
        Ok(r) => {
            debug!("  r={:?}", r);
            let s = String::from_utf8_lossy(r.stdout.as_slice());
            match s.split("/").last().clone() {
                // if the git branch is "" return None
                Some(s) => match s.len() {
                    0 => None,
                    _ => Some(s.trim().to_string())
                },
                None => None
            }
        },
        Err(_) => None
    }
}
