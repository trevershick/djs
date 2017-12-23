// the point of this module is to encapsulate all the
// jenkins information and querying, etc...
//
extern crate serde_xml_rs;
extern crate reqwest;

use self::serde_xml_rs::deserialize;
use djs::config::Config;
use std::convert::From;

#[derive(Debug)]
pub struct Jenkins<'a> {
    pub config : &'a mut Config,
    resolved_download_url : Option<String>
}

#[derive(Debug, Deserialize)]
struct JNumber {
   #[serde(rename = "$value")]
   pub number : String
}
#[derive(Debug, Deserialize)]
struct JRelativePath {
   #[serde(rename = "$value")]
   pub value : String
}


impl<'a> Jenkins<'a> {
    pub fn new(config: &mut Config) -> Jenkins {
        Jenkins { config: config, resolved_download_url: None }
    }

    fn config(&self) -> &Config {
        // this is unsafe but i want to panic if this is not set
        self.config
    }
    //http://localhost:8080/job/Tanium/job/discover/job/master/api/xml?depth=2&tree=builds[number,keepLog]&xpath=/*/build[keepLog=%22true%22]/number
    //
    fn build_number_for_last_successful(&self) -> Result<i32, String> {
        let c = self.config();
        let url = format!("{url}/{base}/job/{project}/job/{branch}/lastSuccessfulBuild/api/xml?xpath=/*/number",
                url = c.url,
                base = c.base,
                project = c.project,
                branch = c.branch);

        debug!("Downloading {}", url);

        let client = reqwest::Client::new();
        let res = client.get(url.as_str())
//            .header(UserAgent::new("foo"))
            .send()
            .unwrap();

        let n_r : Result<JNumber,_> = deserialize(res);
        if let Ok(n) = n_r {
            match n.number.parse::<i32>() {
                Ok(v) => return Ok(v),
                Err(_) => return Err(String::from("Couldn't parse value"))
            }
        }
        Err(String::from("Couldn't parse result"))
        // returns <number>3</number>
        // returns 404 or 200
        //Err(String::from("Unable to resolve lastSuccessfulBuild build."))
    }

    pub fn download(&mut self) -> Result<(), String> {
        let resolved = self.resolve_download_url();
        if resolved.is_err() {
            return Err(resolved.err().unwrap());
        }

        Ok(())
    }

    fn find_artifact_path(&self, build_num : i32) -> Result<String, String> {
        // given a buiild number, find the solution file
//    http://localhost:8080/job/Tanium/job/discover/job/master/3/api/xml?xpath=/*/artifact[fileName=%22output.txt%22]/relativePath


        let c = self.config();
        let url = format!("{url}/{base}/job/{project}/job/{branch}/{buildnumber}/api/xml?xpath=/*/artifact[fileName=%22{solution}.xml%22]/relativePath",
                url = c.url,
                base = c.base,
                project = c.project,
                branch = c.branch,
                buildnumber = build_num,
                solution = c.solution);

        debug!("Downloading {}", url);

        let client = reqwest::Client::new();
        let res = client.get(url.as_str())
//            .header(UserAgent::new("foo"))
            .send()
            .unwrap();

        let n_r : Result<JRelativePath,_> = deserialize(res);
        match n_r {
            Ok(rp) => Ok(rp.value),
            Err(_) => Err(String::from("Couldn't deserialize the artifact path"))
        }
    }

    fn resolve_download_url(&mut self) -> Result<String, String> {
        if let Some(ref x) = self.resolved_download_url {
            debug!("Already resolved, returning existing url {}", x);
            return Ok(x.clone());
        }

        let build_number = self.resolve_build_number();

        if build_number.is_err() {
            return Err(build_number.err().unwrap());
        }


        let bn = build_number.unwrap();
        debug!("Build Number is {:?}", bn);

        debug!("Update configuration with build number");
        self.config.build = bn.to_string();

        let relative_path_to_artifact = self.find_artifact_path(bn);
        if relative_path_to_artifact.is_err() {
            return Err(relative_path_to_artifact.err().unwrap());
        }
        let relPath = relative_path_to_artifact.unwrap();

        debug!("Artifact path is {}", relPath);


       // http://localhost:8080/job/Tanium/job/discover/job/master/lastSuccessfulBuild/artifact/output.txt
        let tmp = format!("{url}/{base}/job/{project}/job/{branch}/{build}/artifact/{a}",
                url = self.config().url,
                base = self.config().base,
                project = self.config().project,
                branch = self.config().branch,
                build = self.config().build,
                a = relPath);
        debug!("Resolved URL is {}", tmp);

       self.resolved_download_url = Some(tmp.clone());
       Ok(tmp)
    }

    fn build_number_from_build(&self) -> Result<i32, String> {
        match self.config().build.parse::<i32>() {
            Ok(num) => Ok(num),
            Err(_) => Err(format!("{build} is not a valid integer.", build = self.config().build))
        }
    }

    // entry point. return an int build number from either a
    // 'lastSuccessful' build or a number as a string
    fn resolve_build_number(&self) -> Result<i32, String> {
        match self.config().build.as_ref() {
            "lastSuccessfulBuild" => self.build_number_for_last_successful(),
            value => self.build_number_from_build()
        }
    }
}
