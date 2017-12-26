// the point of this module is to encapsulate all the
// jenkins information and querying, etc...
//
//extern crate reqwest;
extern crate indicatif;
extern crate console;
mod xml;

// import this so e.description() works
use reqwest;
use std::error::Error;
use djs::config::Config;
use djs::jenkins::xml::{cdata_i32, cdata_string};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Jenkins {
    config : Rc<RefCell<Config>>,
    resolved_download_url : Option<String>
}

fn get(url: String) -> Result<reqwest::Response, String> {
    reqwest::Client::builder().gzip(false)
        .build()
        .unwrap()
        .get(url.as_str())
        .header(reqwest::header::AcceptEncoding(vec![]))
        .send()
        .map_err(&string_from_response_err)
        .and_then(&successful_response_only)
}


fn successful_response_only(it: reqwest::Response) -> Result<reqwest::Response, String> {
    if it.status().is_success() {
        Ok(it)
    } else {
        Err(format!("{} {:?}", it.url(), it.status()))
    }
}

fn string_from_response_err(it: reqwest::Error) -> String {
    String::from(it.description())
}

impl Jenkins {
    pub fn new(config: Rc<RefCell<Config>>) -> Jenkins {
        Jenkins { config: config, resolved_download_url: None }
    }

    fn build_number_for_last_keep(&self) -> Result<i32, String> {
        debug!("build_number_for_last_keep, build={:?}", self.config.borrow().build.get());

        let c = self.config.borrow();
        let url = format!("{url}/{base}/job/{project}/job/{branch}/api/xml?depth=2&tree=builds[number,keepLog]&xpath=/*/build[keepLog=%22true%22][1]/number",
                url = c.url.get(),
                base = c.base.get(),
                project = c.project.get(),
                branch = c.branch.get());

        debug!("  url={}", url);

        get(url).and_then(&cdata_i32).map_err(|e| format!("Unable to resolve the last \"keep forever\" build\n{}", e))
    }


    fn build_number_for_last_successful(&self) -> Result<i32, String> {
        debug!("build_number_for_last_successful, build={:?}", self.config.borrow().build);

        let c = self.config.borrow();
        let url = format!("{url}/{base}/job/{project}/job/{branch}/lastSuccessfulBuild/api/xml?xpath=/*/number",
                url = c.url.get(),
                base = c.base.get(),
                project = c.project.get(),
                branch = c.branch.get());

        debug!("  url={}", url);

        get(url).and_then(&cdata_i32).map_err(|e| format!("Unable to resolve the last successful build\n{}", e))
    }

    /// given a build number and the current config, find the relative path to the artifact
    fn find_artifact_path(&self, build_num : i32) -> Result<String, String> {
        let c = self.config.borrow();
        let url = format!("{url}/{base}/job/{project}/job/{branch}/{buildnumber}/api/xml?xpath=/*/artifact[fileName=%22{solution}%22]/relativePath",
                url = c.url.get(),
                base = c.base.get(),
                project = c.project.get(),
                branch = c.branch.get(),
                buildnumber = build_num,
                solution = c.solution.get());

        debug!("Downloading {}", url);

        get(url).and_then(&cdata_string)
    }

    fn update_build_with(&self, bn: i32) {
        debug!("Update configuration with build number {:?}", bn);
        let old_build_number = self.config.borrow().build.get();

        let mut src = self.config.borrow().build.source();
        src = format!("jenkins, was {} from {}", old_build_number, src);

        self.config.borrow_mut().build.set(bn.to_string(), src);
    }

    pub fn resolve_download_url(&mut self) -> Result<String, String> {
        if let Some(ref x) = self.resolved_download_url {
            debug!("Already resolved, returning existing url {}", x);
            return Ok(x.clone());
        }

        let build_number = self.resolve_build_number();

        let bn = match build_number {
            Ok(v) => v,
            Err(x) => return Err(x)
        };


        let relative_path_to_artifact = self.find_artifact_path(bn);
        if relative_path_to_artifact.is_err() {
            return Err(relative_path_to_artifact.err().unwrap());
        }
        let rel_path = relative_path_to_artifact.unwrap();

        debug!("Artifact path is {}", rel_path);

        let tmp = format!("{url}/{base}/job/{project}/job/{branch}/{build}/artifact/{a}",
                url = self.config.borrow().url.get(),
                base = self.config.borrow().base.get(),
                project = self.config.borrow().project.get(),
                branch = self.config.borrow().branch.get(),
                build = self.config.borrow().build.get(),
                a = rel_path);
        debug!("Resolved URL is {}", tmp);

       self.resolved_download_url = Some(tmp.clone());
       Ok(tmp)
    }

    /// Expecting the config.build number to be an integer (build number)
    /// if it's not return an error
    fn build_number_from_build(&self) -> Result<i32, String> {
        debug!("build_number_from_build, build={:?}", self.config.borrow().build);
        match self.config.borrow().build.get().parse::<i32>() {
            Ok(num) => Ok(num),
            Err(_) => Err(format!("{build} is not a valid integer.", build = self.config.borrow().build))
        }
    }

    // entry point. return an int build number from either a
    // 'lastSuccessful' build or a number as a string
    fn resolve_build_number(&self) -> Result<i32, String> {
        debug!("resolve_build_number, build={:?}", self.config.borrow().build);
        let x = self.config.borrow().build.get();
        match x.as_ref() {
            "lastSuccessfulBuild" => {
                self.build_number_for_last_successful().and_then(|bn| {
                    self.update_build_with(bn);
                    Ok(bn)
                })
            },
            "lastKeepForever" => {
                self.build_number_for_last_keep().and_then(|bn| {
                    self.update_build_with(bn);
                    Ok(bn)
                })
            },
            _ => self.build_number_from_build()
        }
    }
}


