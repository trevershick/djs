
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
                Some(s) => return Some(String::from(s.trim())),
                None => return None
            }
        },
        Err(_) => None
    }
}
