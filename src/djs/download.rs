use console::{style};

use reqwest;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use djs::mediator::Mediator;

pub fn download(url: &str, fname: &str, mediator: &mut Mediator) -> Result<(), Box<::std::error::Error>> {

    // parse url
    let client = reqwest::Client::builder()
        .gzip(false)
        .build()?;

    let mut resp = client.get(url)
        .header(reqwest::header::AcceptEncoding(vec![]))
        //.danger_disable_hostname_verification()
        .send().unwrap();
    mediator.print(format!("HTTP request sent... {}", style(format!("{}", resp.status())).green()));
    if resp.status().is_success() {

        let headers = resp.headers().clone();
        debug!("headers = {:?}", headers);

        let ct_len = headers.get::<reqwest::header::ContentLength>().map(|it| it.0);
        debug!("ct_len = {:?}", ct_len);

        match ct_len {
            Some(len) => {
                mediator.print(format!("Length: {} ({})",
                      style(len).green(),
                      style(format!("{}", mediator.human_bytes(len))).red()));
            },
            None => {
                mediator.print(format!("Length: {}", style("unknown").red()));
            },
        }

        mediator.print(format!("Saving to: {}", style(fname).green()));

        mediator.start_progress(fname, ct_len);

        let mut file = File::create(fname)?;

        let mut buffer = [0; 8192];
        let mut bcount: usize;
        loop {
            bcount = resp.read(&mut buffer)?;
            if bcount == 0 {
                break;
            }
            mediator.incr_progress(fname, bcount as u64);
            file.write_all(&buffer[..bcount])?;
        }
        mediator.finish_progress(fname);
    }
    Ok(())
}

