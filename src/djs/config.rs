#[allow(unused_imports)]
use djs::defaults::*;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub url: String,
    pub base: String,
    pub project: String,

    pub branch: String,
    pub build: String,

    pub solution: String,
    pub destination: String
}

impl Config {
    fn is_destination_a_dir(&self) -> bool {
        match fs::metadata(self.destination.clone()) {
            Ok(m) => m.is_dir(),
            Err(_) => false
        }
    }

    fn is_destination_writable(&self) -> bool {
        match fs::metadata(self.destination.clone()) {
            Ok(m) => m.permissions().readonly(),
            Err(_) => true
        }
    }

    pub fn destination_path(&self) -> String {
        // if the destination is a dir, then we build it
        // if the destination is a filename return it
        let default_dest = format!("{destination}/{branch}-{build}-{project}.xml",
                destination = self.destination,
                branch = self.branch,
                build = self.build,
                project = self.project);

        let dest_is_dir = match fs::metadata(self.destination.clone()) {
            Ok(meta) => meta.is_dir() && !meta.permissions().readonly(),
            Err(_) => false
        };

        if dest_is_dir {
            default_dest
        } else {
            self.destination.clone()
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            url: String::from(DEFAULT_URL),
            base: String::from(DEFAULT_BASE),
            project: String::from(DEFAULT_PROJECT),

            branch: String::from(DEFAULT_BRANCH),
            build: String::from(DEFAULT_BUILD),

            solution: String::from(DEFAULT_SOLUTION),
            destination: String::from(DEFAULT_DESTINATION)
        }
    }
}


pub fn validate_config(config : &Config) -> Result<(), String> {
    /*
    if config.solution.is_none() {
       return Err(String::from("solution is required"));
    }
    if config.solutionpath.is_none() {
       return Err(String::from("solutionpath is required"));
    }
    */
    Ok(())
}
