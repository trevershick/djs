use std::fmt;
use std::error::Error;
use console::style;
use reqwest;

#[derive(Debug)]
pub enum DjsError {
    InvalidDestinationTemplate(String),
    // option, current value, notes
    InvalidConfig(String, String, String),
    HttpError(reqwest::Error),
    HttpRequestFailed(String, String),
    XmlContentError(String,String /*bad value*/),
    // solution, url
    ArtifactNotFound(String, String),
    EmptyContentError,
    DownloadFailure(String, String, Box<::std::error::Error>),
    // what step, cause
    StepFailed(String, Box<DjsError>),
}

impl DjsError {
    pub fn step_failed<S>(step: S, why: DjsError) -> DjsError
    where
        S: Into<String>,
    {
        DjsError::StepFailed(step.into(), Box::new(why))
    }

    pub fn download_failure<S>(url: S, fname: S, e: Box<Error>) -> DjsError
    where
        S: Into<String>,
    {
        DjsError::DownloadFailure(url.into(), fname.into(), e)
    }
}

impl fmt::Display for DjsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DjsError::InvalidDestinationTemplate(ref fmt) => {
                write!(f, "{} {}", "Invalid Format", style(fmt).red())
            }
            DjsError::InvalidConfig(ref opt, ref cur_val, ref notes) => write!(
                f,
                "{opt} \"{cur_val}\" is invalid. Hint: {notes}",
                opt = style(opt).green(),
                cur_val = style(cur_val).red(),
                notes = style(notes)
            ),
            DjsError::HttpError(ref re) => write!(f, "An HTTP Error Occured {re}", re = re),
            DjsError::HttpRequestFailed(ref url, ref status) => write!(
                f,
                "{c} - {u}",
                c = style(status).red(),
                u = style(url).green()
            ),
            DjsError::XmlContentError(ref note, ref bad_value) => write!(
                f,
                "An XML Content error occured : {} for \"{}\"",
                style(note).red(),
                style(bad_value).red()
            ),
            DjsError::EmptyContentError => write!(f, "No data returned in xml."),
            DjsError::ArtifactNotFound(ref a, ref url) => write!(
                f,
                "Unable to find the artifact {a}. --v\n    {u}",
                a = style(a).red(),
                u = style(url).green()
            ),
            DjsError::DownloadFailure(ref url, ref _dest, ref error) => write!(
                f,
                "Downloading {u} failed: {e}",
                u = style(url).green(),
                e = error
            ),
            DjsError::StepFailed(ref step, ref nested) => write!(
                f,
                "{step} failed due to ----v\n    {nested}",
                step = step,
                nested = nested
            ),
        }
    }
}

impl Error for DjsError {
    fn description(&self) -> &str {
        match *self {
            DjsError::InvalidDestinationTemplate(..) => "Invalid Destination Template",
            DjsError::InvalidConfig(..) => "Invalid Config",
            DjsError::HttpError(..) => "HTTP Request Error",
            DjsError::HttpRequestFailed(..) => "HTTP Request Failed",
            DjsError::XmlContentError(..) => "XML Content Error",
            DjsError::DownloadFailure(..) => "Download Failure",
            DjsError::ArtifactNotFound(..) => "Can't find artifact.",
            DjsError::EmptyContentError => "No data returned in xml.",
            DjsError::StepFailed(_, ref nested) => nested.description(),
        }
    }
}
