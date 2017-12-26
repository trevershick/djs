#[allow(unused_imports)]
use djs::defaults::*;
use std::fs;
use std;
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

    pub fn set(&mut self, value : T, source: String) {
        self.value = value;
        self.source = source;
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

    pub fn destination_path(&self) -> String {
        // if the destination is a dir, then we build it
        // if the destination is a filename return it
        let default_dest = format!("{destination}/{branch}-{build}-{project}.xml",
                destination = self.destination.get(),
                branch = self.branch.get(),
                build = self.build.get(),
                project = self.project.get());

        let dest_is_dir = match fs::metadata(self.destination.get()) {
            Ok(meta) => meta.is_dir() && !meta.permissions().readonly(),
            Err(_) => false
        };

        if dest_is_dir {
            default_dest
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

    Ok(())
}
