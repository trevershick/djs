#[allow(unused_imports)]
#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate clap;
extern crate reqwest;

mod djs;

use std::io::{stderr, Write};
use std::process::{exit};
use djs::config::{Config, validate_config};
use djs::cli::{configure_from_cli, build_cli};
use djs::rc::{configure_from_file};
use djs::jenkins::Jenkins;


fn main() {
    env_logger::init();
    let cli = build_cli();
	let opts = cli.get_matches();
    let mut c = Config {..Default::default()};
    debug!("Start State, c = {:?}", c);

    // read from file
    // override from command line
    //c.solution = Some(String::from("Discover"));

    configure_from_file(&mut c).expect("Failed to configure from the file.");
    debug!("About to configure from CLI");
    configure_from_cli(&mut c, &opts).expect("Failed to parse the CLI");


    if let Some(err) = validate_config(&c).err() {
        writeln!(stderr(), "{:?}", err).unwrap();
        exit(1)
    }

    let mut j = Jenkins::new(&mut c);
    debug!("Jenkins = {:?}", j);

    if let Some(err) = j.download().err() {
        writeln!(stderr(), "{:?}", err).unwrap();
        exit(1)
    }

    /*println!("Download {url} to {d}",
             url = j.download_url(),
             d = c.destination_path());*/
    exit(0)
}
