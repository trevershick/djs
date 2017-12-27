// the point of this module is to encapsulate all the
// jenkins information and querying, etc...
//
//extern crate reqwest;
extern crate console;
extern crate indicatif;
//
mod xml;

// import this so e.description() works
use reqwest;
use djs::error::DjsError;
use djs::config::Config;
use djs::jenkins::xml::{cdata_string, cdata_i32};
use std::rc::Rc;
use std::time::Duration;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Jenkins {
    config: Rc<RefCell<Config>>,
    resolved_download_url: Option<String>,
}

impl From<reqwest::Error> for DjsError {
    fn from(err: reqwest::Error) -> DjsError {
        DjsError::HttpError(err)
    }
}

fn get(url: String) -> Result<reqwest::Response, DjsError> {
    reqwest::Client::builder()
        .gzip(false)
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap()
        .get(url.as_str())
        .header(reqwest::header::AcceptEncoding(vec![]))
        .send()
        .map_err(|e| From::from(e))
        .and_then(&successful_response_only)
}

fn successful_response_only(it: reqwest::Response) -> Result<reqwest::Response, DjsError> {
    if it.status().is_success() {
        Ok(it)
    } else {
        Err(DjsError::HttpRequestFailed(
            it.url().to_string(),
            format!("{}", it.status()),
        ))
    }
}

impl Jenkins {
    pub fn new(config: Rc<RefCell<Config>>) -> Jenkins {
        Jenkins {
            config: config,
            resolved_download_url: None,
        }
    }

    fn build_number_for_last_keep(&self) -> Result<i32, DjsError> {
        debug!(
            "build_number_for_last_keep, build={:?}",
            self.config.borrow().build.get()
        );

        let c = self.config.borrow();
        let url = format!("{url}/{base}/job/{project}/job/{branch}/api/xml?depth=2&tree=builds[number,keepLog]&xpath=/*/build[keepLog=%22true%22][1]/number&wrapper=x",
                url = c.url.get(),
                base = c.base.get(),
                project = c.project.get(),
                branch = c.branch.get());

        debug!("  url={}", url);

        get(url).and_then(&cdata_i32).map_err(|e| {
            DjsError::step_failed("Unable to resolve the last \"keep forever\" build", e)
        })
    }

    fn build_number_for_latest_url(&self) -> String {
        debug!(
            "build_number_for_latest_url, build={:?}",
            self.config.borrow().build
        );

        let c = self.config.borrow();
        format!(
            "{url}/{base}/job/{project}/job/{branch}/api/xml?xpath=/*/build/number&wrapper=x",
            url = c.url.get(),
            base = c.base.get(),
            project = c.project.get(),
            branch = c.branch.get()
        )
    }

    fn build_number_for_last_successful_url(&self) -> String {
        debug!(
            "build_number_for_last_successful_url, build={:?}",
            self.config.borrow().build
        );

        let c = self.config.borrow();
        format!("{url}/{base}/job/{project}/job/{branch}/lastSuccessfulBuild/api/xml?xpath=/*/number&wrapper=x",
                url = c.url.get(),
                base = c.base.get(),
                project = c.project.get(),
                branch = c.branch.get())
    }

    fn build_number_for_latest(&self) -> Result<i32, DjsError> {
        debug!(
            "build_number_for_latest, build={:?}",
            self.config.borrow().build
        );
        let url = self.build_number_for_latest_url();

        get(url)
            .and_then(&cdata_i32)
            .map_err(|e| DjsError::step_failed("Unable to resolve the latest build", e))
    }

    fn build_number_for_last_successful(&self) -> Result<i32, DjsError> {
        debug!(
            "build_number_for_last_successful, build={:?}",
            self.config.borrow().build
        );
        let url = self.build_number_for_last_successful_url();

        get(url).and_then(&cdata_i32).map_err(|e: DjsError| {
            DjsError::step_failed("Unable to resolve the last successful build", e)
        })
    }

    fn find_artifact_path_url(&self, build_num: i32) -> String {
        let c = self.config.borrow();

        let mut q = "xpath=/*/artifact[fileName=%22".to_string();
        q.push_str(c.solution.get().as_str());
        q.push_str("%22");

        if c.solution_filter.get().len() > 0 {
            q.push_str(" and contains(relativePath, %22");
            q.push_str(c.solution_filter.get().as_str());
            q.push_str("%22)");
        }

        q.push_str("]/relativePath");
        q.push_str("&wrapper=x");

        format!(
            "{url}/{base}/job/{project}/job/{branch}/{buildnumber}/api/xml?{q}",
            url = c.url.get(),
            base = c.base.get(),
            project = c.project.get(),
            branch = c.branch.get(),
            buildnumber = build_num,
            q = q
        )
    }

    /// given a build number and the current config, find the relative path to the artifact
    fn find_artifact_path(&self, build_num: i32) -> Result<String, DjsError> {
        let url = self.find_artifact_path_url(build_num);
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

    pub fn resolve_download_url(&mut self) -> Result<String, DjsError> {
        if let Some(ref x) = self.resolved_download_url {
            debug!("Already resolved, returning existing url {}", x);
            return Ok(x.clone());
        }

        let build_number = self.resolve_build_number();

        let bn = match build_number {
            Ok(v) => v,
            Err(x) => return Err(x),
        };

        let relative_path_to_artifact = self.find_artifact_path(bn);
        if relative_path_to_artifact.is_err() {
            return Err(relative_path_to_artifact.err().unwrap());
        }
        let rel_path = relative_path_to_artifact.unwrap();

        debug!("Artifact path is {}", rel_path);

        let tmp = format!(
            "{url}/{base}/job/{project}/job/{branch}/{build}/artifact/{a}",
            url = self.config.borrow().url.get(),
            base = self.config.borrow().base.get(),
            project = self.config.borrow().project.get(),
            branch = self.config.borrow().branch.get(),
            build = self.config.borrow().build.get(),
            a = rel_path
        );
        debug!("Resolved URL is {}", tmp);

        self.resolved_download_url = Some(tmp.clone());
        Ok(tmp)
    }

    /// Expecting the config.build number to be an integer (build number)
    /// if it's not return an error
    fn build_number_from_build(&self) -> Result<i32, DjsError> {
        debug!(
            "build_number_from_build, build={:?}",
            self.config.borrow().build
        );
        match self.config.borrow().build.get().parse::<i32>() {
            Ok(num) => Ok(num),
            Err(_) => Err(DjsError::InvalidConfig(
                format!("build"),
                self.config.borrow().build.get(),
                format!("It should be an integer value."),
            )),
        }
    }

    // entry point. return an int build number from either a
    // 'lastSuccessful' build or a number as a string
    fn resolve_build_number(&self) -> Result<i32, DjsError> {
        debug!(
            "resolve_build_number, build={:?}",
            self.config.borrow().build
        );
        let x = self.config.borrow().build.get();
        match x.as_ref() {
            "latest" => self.build_number_for_latest().and_then(|bn| {
                self.update_build_with(bn);
                Ok(bn)
            }),
            "lastSuccessfulBuild" => self.build_number_for_last_successful().and_then(|bn| {
                self.update_build_with(bn);
                Ok(bn)
            }),
            "lastKeepForever" => self.build_number_for_last_keep().and_then(|bn| {
                self.update_build_with(bn);
                Ok(bn)
            }),
            _ => self.build_number_from_build(),
        }
    }
}
