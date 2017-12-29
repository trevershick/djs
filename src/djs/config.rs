extern crate strfmt;
#[allow(unused_imports)]
extern crate url;


use std::error::Error;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::{self, Display, Formatter};

use djs::defaults::*;
use djs::error::DjsError;

use self::strfmt::strfmt;
use self::url::Url;

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
    pub destination_template: ConfigValue<String>,
    pub dry_run: ConfigValue<bool>,
    pub verbose: ConfigValue<bool>,
    pub quiet: ConfigValue<bool>,
}

#[derive(Debug, Clone)]
pub struct ConfigValue<T: Clone> {
    value: T,
    source: String,
}

macro_rules! get_c { ($c:ident, $o: ident) => { $c.$o.get().clone() } }

impl Display for ConfigValue<String> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} [from {}]", self.get(), self.source())
    }
}

impl<T: Clone> ConfigValue<T> {
    fn new(value: T, source: String) -> ConfigValue<T> {
        ConfigValue {
            value: value,
            source: source,
        }
    }

    pub fn set<S>(&mut self, value: T, source: S)
    where
        S: Into<String>,
    {
        self.value = value;
        self.source = source.into();
    }

    ///
    /// Set the value and the source if and only if the current source matches 'existing_source'
    /// this can be used to only override the value if the current source is 'defaults'
    ///
    pub fn set_if_source<S, E>(&mut self, value: T, source: S, existing_source: E)
    where
        S: Into<String>,
        E: Into<String>,
    {
        if self.source == existing_source.into().as_ref() {
            self.value = value;
            self.source = source.into();
        }
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
            Err(_) => false,
        }
    }

    fn is_destination_writable(&self) -> bool {
        match fs::metadata(self.destination.get()) {
            Ok(m) => !m.permissions().readonly(),
            Err(_) => true,
        }
    }

    fn branch_name_contains_project(&self) -> bool {
        self.branch
            .get()
            .to_uppercase()
            .as_str()
            .contains(self.project.get().to_uppercase().as_str())
    }

    fn abbreviated_build(&self) -> String {
        match self.build.get().trim() {
            "lastSuccessfulBuild" => "ls".to_string(),
            "latest" => "lt".to_string(),
            "lastKeepForever" => "kf".to_string(),
            other => other.to_string(),
        }
    }

    pub fn destination_path(&self) -> String {
        // if the destination is a dir, then we build it
        // if the destination is a filename return it
        let dest_is_dir = match fs::metadata(self.destination.get()) {
            Ok(meta) => meta.is_dir() && !meta.permissions().readonly(),
            Err(_) => false,
        };

        if dest_is_dir {
            let mut d = self.destination.get().clone();
            d.push('/');

            let custom = self.destination_template.get();
            let file_name = match custom.len() {
                0 => self.default_destination_path(),
                _ => self.custom_destination_path().unwrap(),
            };
            d.push_str(file_name.as_str());
            d
        } else {
            self.destination.get()
        }
    }

    fn branch_nums_only(&self) -> String {
        let v = self.branch
            .get()
            .chars()
            .filter(|c| *c >= '0' && *c <= '9')
            .collect::<String>();
        match v.len() {
            0 => self.branch.get(),
            _ => v,
        }
    }

    fn branch_alpha_only(&self) -> String {
        let v = self.branch
            .get()
            .chars()
            .filter(|c| (*c >= 'a' && *c <= 'z') || (*c >= 'A' && *c <= 'Z'))
            .collect::<String>();
        match v.len() {
            0 => self.branch.get(),
            _ => v,
        }
    }

    fn solution_extension(&self) -> Option<String> {
        Path::new(self.solution.get().as_str())
            .extension()
            .and_then(|o| o.to_str())
            .map(|o| o.to_string())
    }

    fn solution_basename(&self) -> Option<String> {
        Path::new(&self.solution.get())
            .file_stem()
            .and_then(|o| o.to_str())
            .map(|o| o.to_string())
    }

    fn custom_destination_path(&self) -> Result<String, DjsError> {
        let tmpl = self.destination_template.get();
        let branch_nums = self.branch_nums_only();
        let branch_alphas = self.branch_alpha_only();
        let solution_basename = self.solution_basename().unwrap_or(s!(""));
        let solution_extension = self.solution_extension().unwrap_or(s!(""));

        let vars = map!{
            s!("project") => get_c!(self,project).to_lowercase(),
            s!("branch") => get_c!(self,branch).to_lowercase(),
            s!("branch_nums") => branch_nums.to_lowercase(),
            s!("branch_alphas") => branch_alphas.to_lowercase(),
            s!("build") => get_c!(self,build).to_lowercase(),
            s!("build_abbreviation") => self.abbreviated_build().to_lowercase(),
            s!("solution") => get_c!(self,solution).to_lowercase(),
            s!("solution_basename") => solution_basename.to_lowercase(),
            s!("solution_extension") => solution_extension.to_lowercase(),
            s!("solution_filter") => get_c!(self,solution_filter).to_lowercase(),

            s!("PROJECT") => get_c!(self,project).to_uppercase(),
            s!("BRANCH") => get_c!(self,branch).to_uppercase(),
            s!("BRANCH_NUMS") => branch_nums.to_uppercase(),
            s!("BRANCH_ALPHAS") => branch_alphas.to_uppercase(),
            s!("BUILD") => get_c!(self,build).to_uppercase(),
            s!("BUILD_ABBREVIATION") => self.abbreviated_build().to_uppercase(),
            s!("SOLUTION") => get_c!(self,solution).to_uppercase(),
            s!("SOLUTION_BASENAME") => solution_basename.to_uppercase(),
            s!("SOLUTION_EXTENSION") => solution_extension.to_uppercase(),
            s!("SOLUTION_FILTER") => get_c!(self,solution_filter).to_uppercase(),

            s!("Project") => get_c!(self,project),
            s!("Branch") => get_c!(self,branch),
            s!("Branch_Nums") => branch_nums,
            s!("Branch_Alphas") => branch_alphas,
            s!("Build") => get_c!(self,build),
            s!("Build_Abbreviation") => self.abbreviated_build(),
            s!("Solution") => get_c!(self,solution),
            s!("Solution_Basename") => solution_basename,
            s!("Solution_Extension") => solution_extension,
            s!("Solution_Filter") => get_c!(self,solution_filter)
        };

        strfmt(&tmpl, &vars).map_err(|e| DjsError::InvalidDestinationTemplate(s!(e.description())))
    }

    fn default_destination_path(&self) -> String {
        let mut d = String::from("");
        if !self.branch_name_contains_project() {
            d.push_str(self.project.get().to_lowercase().as_ref());
            d.push('-');
        }

        d.push_str(self.branch.get().to_lowercase().as_ref());
        d.push('-');

        d.push_str(self.abbreviated_build().to_lowercase().as_ref());

        if let Some(ext) = self.solution_extension() {
            d.push('.');
            d.push_str(ext.as_str());
        }
        d
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
            solution_filter: ConfigValue::new(
                String::from(DEFAULT_SOLUTION_FILTER),
                String::from("defaults"),
            ),

            destination: ConfigValue::new(
                String::from(DEFAULT_DESTINATION),
                String::from("defaults"),
            ),
            destination_template: ConfigValue::new(
                String::from(DEFAULT_DESTINATION_TEMPLATE),
                String::from("defaults"),
            ),

            dry_run: ConfigValue::new(false, String::from("defaults")),
            verbose: ConfigValue::new(false, String::from("defaults")),
            quiet: ConfigValue::new(false, String::from("defaults")),
        }
    }
}

pub fn validate_config(config: Rc<RefCell<Config>>) -> Result<(), DjsError> {
    macro_rules! reject_if_blank {
        ($o: ident) => {
            if config.borrow().$o.get().replace(char::is_whitespace, "").len() == 0 {
                return Err(DjsError::InvalidConfig(format!("{}", stringify!($o)),
                                           config.borrow().$o.get(),
                                           format!("It is blank.")));
            }
        }
    }

    reject_if_blank!(url);
    reject_if_blank!(project);
    reject_if_blank!(branch);
    reject_if_blank!(build);
    reject_if_blank!(solution);
    reject_if_blank!(destination);

    let c = config.borrow();

    if Url::parse(c.url.get().as_ref()).is_err() {
        return Err(DjsError::InvalidConfig(
            "url".to_string(),
            c.url.get(),
            "Fix the URL".to_string(),
        ));
    }

    if c.is_destination_a_dir() && !c.is_destination_writable() {
        return Err(DjsError::InvalidConfig(
            "description".to_string(),
            c.destination.get(),
            "The destination directory is not writable.".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    macro_rules! set_c {
        ($config: ident, $opt:ident, $val: expr) => {
            $config.$opt.set($val.to_string(), "test");
        }
    }

    macro_rules! config {
        () => {
            Config {
                ..Default::default()
            };
        }
    }

    mod abbreviated_build {
        use super::super::*;

        #[test]
        fn last_successful() {
            let mut c = config!();
            set_c!(c, build, "lastSuccessfulBuild");
            assert_eq!("ls", c.abbreviated_build());
        }
        #[test]
        fn last_keep_forever() {
            let mut c = config!();
            set_c!(c, build, "lastKeepForever");
            assert_eq!("kf", c.abbreviated_build());
        }
        #[test]
        fn latest() {
            let mut c = config!();
            set_c!(c, build, "latest");
            assert_eq!("lt", c.abbreviated_build());
        }
        #[test]
        fn other() {
            let mut c = config!();
            set_c!(c, build, "xxx");
            assert_eq!("xxx", c.abbreviated_build());
        }
    }

    mod destination_template {
        use super::super::*;
        #[test]
        fn basic() {
            let mut c = config!();
            set_c!(c, destination_template, "scooby");
            set_c!(c, solution, "file.txt");
            assert_eq!(c.destination_path().as_str(), "./scooby");
        }

        #[test]
        fn branch() {
            let mut c = config!();
            set_c!(c, destination_template, "{Branch}");
            set_c!(c, branch, "Shaggy");
            set_c!(c, solution, "file.txt");
            assert_eq!(c.destination_path().as_str(), "./Shaggy");
        }

        #[test]
        fn lc_branch() {
            let mut c = config!();
            set_c!(c, destination_template, "{branch}");
            set_c!(c, branch, "Shaggy");
            set_c!(c, solution, "file.txt");
            assert_eq!(c.destination_path().as_str(), "./shaggy");
        }

        #[test]
        fn uc_branch() {
            let mut c = config!();
            set_c!(c, destination_template, "{BRANCH}");
            set_c!(c, branch, "Shaggy");
            set_c!(c, solution, "file.txt");
            assert_eq!(c.destination_path().as_str(), "./SHAGGY");
        }

        #[test]
        fn abbreviation() {
            let mut c = config!();
            set_c!(c, destination_template, "{build_abbreviation}");
            set_c!(c, build, "latest");
            set_c!(c, solution, "file.txt");
            assert_eq!(c.destination_path().as_str(), "./lt");
        }

        #[test]
        fn combined_lc() {
            let mut c = config!();
            set_c!(c, destination_template, "{project}-{solution_basename}-{solution_filter}-{branch}-{build}-{build_abbreviation}.{solution_extension}");
            set_c!(c, project, "proj1");
            set_c!(c, solution, "solution.txt");
            set_c!(c, solution_filter, "filter1");
            set_c!(c, branch, "branch1");
            set_c!(c, build, "latest");
            assert_eq!(
                c.destination_path().as_str(),
                "./proj1-solution-filter1-branch1-latest-lt.txt"
            );

            set_c!(c, build, "14");
            assert_eq!(
                c.destination_path().as_str(),
                "./proj1-solution-filter1-branch1-14-14.txt"
            );
        }

        #[test]
        fn combined_uc() {
            let mut c = config!();
            set_c!(c, destination_template, "{PROJECT}-{SOLUTION_BASENAME}-{SOLUTION_FILTER}-{BRANCH}-{BUILD}-{BUILD_ABBREVIATION}.{SOLUTION_EXTENSION}");
            set_c!(c, project, "proj1");
            set_c!(c, solution, "solution.txt");
            set_c!(c, solution_filter, "filter1");
            set_c!(c, branch, "branch1");
            set_c!(c, build, "latest");
            assert_eq!(
                c.destination_path().as_str(),
                "./PROJ1-SOLUTION-FILTER1-BRANCH1-LATEST-LT.TXT"
            );
        }

        #[test]
        fn combined_preserve() {
            let mut c = config!();
            set_c!(c, destination_template, "{Project}-{Solution_Basename}-{Solution_Filter}-{Branch}-{Build}-{Build_Abbreviation}.{Solution_Extension}");
            set_c!(c, project, "Proj1");
            set_c!(c, solution, "Solution.Txt");
            set_c!(c, solution_filter, "Filter1");
            set_c!(c, branch, "Branch1");
            set_c!(c, build, "latest");
            assert_eq!(
                c.destination_path().as_str(),
                "./Proj1-Solution-Filter1-Branch1-latest-lt.Txt"
            );
        }
    }

    mod branch_alpha_only {
        use super::super::*;

        #[test]
        fn when_there_are_numbers() {
            let mut c = config!();
            set_c!(c, branch, "MyBranch-14");
            assert_eq!(c.branch_alpha_only(), "MyBranch");
        }

        #[test]
        fn when_there_are_no_letters() {
            let mut c = config!();
            set_c!(c, branch, "144");
            assert_eq!(c.branch_nums_only(), "144");
        }
    }

    mod branch_nums_only {
        use super::super::*;

        #[test]
        fn when_there_are_numbers() {
            let mut c = config!();
            set_c!(c, branch, "MyBranch-14");
            assert_eq!(c.branch_nums_only(), "14");
        }

        #[test]
        fn when_there_are_no_numbers() {
            let mut c = config!();
            set_c!(c, branch, "MyBranch");
            assert_eq!(c.branch_nums_only(), "MyBranch");
        }
    }
    mod destination_path {
        use super::super::*;

        #[test]
        fn default_format() {
            let c = config!();
            assert_eq!(c.destination_path().as_str(), "./myproject-master-ls.xml");
        }

        #[test]
        fn doesnt_duplicate_project() {
            let mut c = config!();
            set_c!(c, branch, "MYPROJECT-1814");
            assert_eq!(c.destination_path().as_str(), "./myproject-1814-ls.xml");
        }
    }
}
