// the point of this module is to encapsulate all the
// jenkins information and querying, etc...
//
//extern crate reqwest;
extern crate indicatif;
extern crate console;

// import this so e.description() works
use reqwest;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use self::console::{style};
use djs::progressbar::create_progress_bar;

fn print(out: String, quiet_mode: bool) {
    if !quiet_mode {
        println!("{}", out.as_str());
    }
}

pub fn download(url: &str, fname: &str, quiet_mode: bool) -> Result<(), Box<::std::error::Error>> {

    // parse url
    let client = reqwest::Client::builder()
        .gzip(false)
        .build()?;

    let mut resp = client.get(url)
        .header(reqwest::header::AcceptEncoding(vec![]))
        //.danger_disable_hostname_verification()
        .send().unwrap();
    print(format!("HTTP request sent... {}", style(format!("{}", resp.status())).green()), quiet_mode);
    if resp.status().is_success() {

        let headers = resp.headers().clone();
        debug!("headers = {:?}", headers);

        let ct_len = headers.get::<reqwest::header::ContentLength>().map(|it| it.0);
        debug!("ct_len = {:?}", ct_len);

        let ct_type = headers.get::<reqwest::header::ContentType>().unwrap();

        match ct_len {
            Some(len) => {
                print(format!("Length: {} ({})",
                      style(len).green(),
                      style(format!("{}", /*HumanBytes(*/len/*)*/)).red()),
                    quiet_mode);
            },
            None => {
                print(format!("Length: {}", style("unknown").red()), quiet_mode);
            },
        }

        print(format!("Type: {}", style(ct_type).green()), quiet_mode);
        print(format!("Saving to: {}", style(fname).green()), quiet_mode);

        let bar = create_progress_bar(quiet_mode, fname, ct_len);

        let mut file = File::create(fname)?;

        let mut buffer = [0; 8192];
        let mut bcount: usize = 0;
        loop {
            bcount = resp.read(&mut buffer)?;
            if bcount == 0 {
                break;
            }

            bar.inc(bcount as u64);
            file.write_all(&buffer[..bcount])?;
        }
        bar.finish();
    }
    Ok(())
}

