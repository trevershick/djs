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

#[derive(Debug)]
pub struct Jenkins<'a> {
    config : &'a mut Config,
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

impl<'a> Jenkins<'a> {
    pub fn new(config: &'a mut Config) -> Jenkins<'a> {
        Jenkins { config: config, resolved_download_url: None }
    }

    fn config(&self) -> &Config {
        self.config
    }


    fn build_number_for_last_keep(&self) -> Result<i32, String> {
        debug!("build_number_for_last_keep, build={:?}", self.config().build.get());

        let c = self.config();
        let url = format!("{url}/{base}/job/{project}/job/{branch}/api/xml?depth=2&tree=builds[number,keepLog]&xpath=/*/build[keepLog=%22true%22][1]/number",
                url = c.url.get(),
                base = c.base.get(),
                project = c.project.get(),
                branch = c.branch.get());

        debug!("  url={}", url);

        get(url).and_then(&cdata_i32).map_err(|e| format!("Unable to resolve the last \"keep forever\" build\n{}", e))
    }


    fn build_number_for_last_successful(&self) -> Result<i32, String> {
        debug!("build_number_for_last_successful, build={:?}", self.config().build);

        let c = self.config();
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
        let c = self.config();
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

        debug!("Update configuration with build number {:?}", bn);
        let src = self.config.build.source();
        self.config.build.set(bn.to_string(), src);

        let relative_path_to_artifact = self.find_artifact_path(bn);
        if relative_path_to_artifact.is_err() {
            return Err(relative_path_to_artifact.err().unwrap());
        }
        let rel_path = relative_path_to_artifact.unwrap();

        debug!("Artifact path is {}", rel_path);

        let tmp = format!("{url}/{base}/job/{project}/job/{branch}/{build}/artifact/{a}",
                url = self.config().url.get(),
                base = self.config().base.get(),
                project = self.config().project.get(),
                branch = self.config().branch.get(),
                build = self.config().build.get(),
                a = rel_path);
        debug!("Resolved URL is {}", tmp);

       self.resolved_download_url = Some(tmp.clone());
       Ok(tmp)
    }

    /// Expecting the config.build number to be an integer (build number)
    /// if it's not return an error
    fn build_number_from_build(&self) -> Result<i32, String> {
        debug!("build_number_from_build, build={:?}", self.config().build);
        match self.config().build.get().parse::<i32>() {
            Ok(num) => Ok(num),
            Err(_) => Err(format!("{build} is not a valid integer.", build = self.config().build))
        }
    }

    // entry point. return an int build number from either a
    // 'lastSuccessful' build or a number as a string
    fn resolve_build_number(&self) -> Result<i32, String> {
        debug!("resolve_build_number, build={:?}", self.config().build);
        match self.config().build.get().as_ref() {
            "lastSuccessfulBuild" => self.build_number_for_last_successful(),
            "lastKeepForever" => self.build_number_for_last_keep(),
            _ => self.build_number_from_build()
        }
    }
}


