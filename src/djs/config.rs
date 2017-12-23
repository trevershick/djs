#[allow(unused_imports)]
use djs::defaults::*;

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
    fn merge(&mut self, with: &Config) {
        self.url = with.url.clone();
    }

    pub fn destination_path(&self) -> String {
        format!("{destination}/{branch}-{build}-{project}.xml",
                destination = self.destination,
                branch = self.branch,
                build = self.build,
                project = self.project)
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
