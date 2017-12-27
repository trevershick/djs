#[allow(unused_imports)]
use djs::defaults::*;
use std::fs;
use std;
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Config {
    pub url: ConfigValue<String>,
    pub base: ConfigValue<String>,
    pub project: ConfigValue<String>,

    pub branch: ConfigValue<String>,
    pub build: ConfigValue<String>,

    pub solution: ConfigValue<String>,
    pub solution_filter: ConfigValue<String>,
    pub destination: ConfigValue<String>,
    pub dry_run: ConfigValue<bool>,
    pub verbose: ConfigValue<bool>,
    pub quiet: ConfigValue<bool>
}

#[derive(Debug, Clone)]
pub struct ConfigValue<T: Clone> {
    value: T,
    source: String
}

impl Display for ConfigValue<String> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} [from {}]", self.get(), self.source())
    }
}

impl<T: Clone> ConfigValue<T> {
    fn new(value: T, source: String) -> ConfigValue<T> {
        ConfigValue { value: value, source: source }
    }

    pub fn set<S>(&mut self, value: T, source: S)  where S: Into<String> {
        self.value = value;
        self.source = source.into();
    }

    pub fn get(&self) -> T {
        self.value.clone()
    }

    pub fn source(&self) -> String {
        self.source.clone()
    }
}

impl Config {
    fn is_destination_a_dir(&self) -> bool {
        match fs::metadata(self.destination.get()) {
            Ok(m) => m.is_dir(),
            Err(_) => false
        }
    }

    fn is_destination_writable(&self) -> bool {
        match fs::metadata(self.destination.get()) {
            Ok(m) => !m.permissions().readonly(),
            Err(_) => true
        }
    }

    fn branch_name_contains_project(&self) -> bool {
        self.branch.get().to_uppercase().as_str().contains(self.project.get().to_uppercase().as_str())
    }

    fn abbreviated_build(&self) -> String {
        match self.build.get().trim() {
            "lastSuccessfulBuild" => "ls".to_string(),
            "lastKeepForever" => "kf".to_string(),
            other => other.to_string()
        }
    }

    pub fn destination_path(&self) -> String {
        // if the destination is a dir, then we build it
        // if the destination is a filename return it
        let dest_is_dir = match fs::metadata(self.destination.get()) {
            Ok(meta) => meta.is_dir() && !meta.permissions().readonly(),
            Err(_) => false
        };

        if dest_is_dir {
            let mut d = self.destination.get().clone();
            d.push('/');

            if !self.branch_name_contains_project() {
                d.push_str(self.project.get().to_lowercase().as_ref());
                d.push('-');
            }

            d.push_str(self.branch.get().to_lowercase().as_ref());
            d.push('-');

            d.push_str(self.abbreviated_build().to_lowercase().as_ref());


            if let Some(ext) = Path::new(self.solution.get().as_str()).extension() {
                d.push('.');
                d.push_str(ext.to_str().unwrap());
            }
            d
        } else {
            self.destination.get()
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            url: ConfigValue::new(String::from(DEFAULT_URL), String::from("defaults")),
            base: ConfigValue::new(String::from(DEFAULT_BASE), String::from("defaults")),
            project: ConfigValue::new(String::from(DEFAULT_PROJECT), String::from("defaults")),

            branch: ConfigValue::new(String::from(DEFAULT_BRANCH), String::from("defaults")),
            build: ConfigValue::new(String::from(DEFAULT_BUILD), String::from("defaults")),

            solution: ConfigValue::new(String::from(DEFAULT_SOLUTION), String::from("defaults")),
            solution_filter: ConfigValue::new(String::from(DEFAULT_SOLUTION_FILTER), String::from("defaults")),

            destination: ConfigValue::new(String::from(DEFAULT_DESTINATION), String::from("defaults")),

            dry_run: ConfigValue::new(false, String::from("defaults")),
            verbose: ConfigValue::new(false, String::from("defaults")),
            quiet: ConfigValue::new(false, String::from("defaults")),
        }
    }
}


pub fn validate_config(config : Rc<RefCell<Config>>) -> Result<(), String> {
    let c = config.borrow();
    if c.is_destination_a_dir() && !c.is_destination_writable() {
        return Err(format!("The destination directory {} is not writable.", c.destination.get()));
    }
    // TODO - lots more validation here, URL formats,etc...

    Ok(())
}

#[cfg(test)]
mod tests {
    macro_rules! cset {
        ($config: ident, $opt:ident, $val: expr) => {
            $config.$opt.set($val.to_string(), "test");
        }
    }

    mod abbreviated_build {
        use super::super::*;

        #[test]
        fn last_successful() {
            let mut c = Config { ..Default::default() };
            cset!(c, build, "lastSuccessfulBuild");
            assert_eq!("ls", c.abbreviated_build());
        }
        #[test]
        fn last_keep_forever() {
            let mut c = Config { ..Default::default() };
            cset!(c, build, "lastKeepForever");
            assert_eq!("kf", c.abbreviated_build());
        }
        #[test]
        fn other() {
            let mut c = Config { ..Default::default() };
            cset!(c, build, "master");
            assert_eq!("master", c.abbreviated_build());
        }
    }

    mod destination_path {
        use super::super::*;

        #[test]
        fn default_format() {
            let c = Config { ..Default::default() };
            assert_eq!(c.destination_path().as_str(), "./discover-master-ls.xml");
        }

        #[test]
        fn doesnt_duplicate_project() {
            let mut c = Config { ..Default::default() };
            cset!(c,branch, "DISCOVER-1814");
            assert_eq!(c.destination_path().as_str(), "./discover-1814-ls.xml");
        }
    }
}
