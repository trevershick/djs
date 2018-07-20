use console::style;

use reqwest;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use djs::error::DjsError;
use djs::mediator::Mediator;
use std::time::Duration;
use djs::config::Config;

pub fn download(
    url: &str,
    fname: &str,
    config: &Config,
    mediator: &mut Mediator,
) -> Result<(), DjsError> {
    // parse url
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(config.timeout_in_seconds.get() as u64))
        .gzip(false)
        .build()?;

    let mut resp = client.get(url)
        .header(reqwest::header::AcceptEncoding(vec![]))
        //.danger_disable_hostname_verification()
        .send().unwrap();
    mediator.print(format!(
        "HTTP request sent... {}",
        style(format!("{}", resp.status())).green()
    ));
    if resp.status().is_success() {
        let headers = resp.headers().clone();
        debug!("headers = {:?}", headers);

        let ct_len = headers
            .get::<reqwest::header::ContentLength>()
            .map(|it| it.0);
        debug!("ct_len = {:?}", ct_len);

        match ct_len {
            Some(len) => {
                mediator.print(format!(
                    "Length: {} ({})",
                    style(len).green(),
                    style(format!("{}", mediator.human_bytes(len))).red()
                ));
            }
            None => {
                mediator.print(format!("Length: {}", style("unknown").red()));
            }
        }

        mediator.print(format!("Saving to: {}", style(fname).green()));

        let dp = Path::new(fname);
        let file_name = dp.file_name().unwrap();
        mediator.print(format!("File: {}", style(file_name.to_str().unwrap()).green()));

        mediator.start_progress(file_name.to_str().unwrap(), ct_len);

        let mut file = File::create(fname)
            .map_err(|e| DjsError::download_failure(url, fname, Box::new(e)))
            .unwrap();

        let mut buffer = [0; 8192];
        let mut bcount: usize;
        loop {
            bcount = resp.read(&mut buffer)
                .map_err(|e| DjsError::download_failure(url, fname, Box::new(e)))
                .unwrap();
            if bcount == 0 {
                break;
            }
            mediator.incr_progress(fname, bcount as u64);
            file.write_all(&buffer[..bcount])
                .map_err(|e| DjsError::download_failure(url, fname, Box::new(e)))
                .unwrap();
        }
        mediator.finish_progress(fname);
    }
    Ok(())
}
