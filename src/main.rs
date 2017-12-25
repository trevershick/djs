#[allow(unused_imports)]
#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate clap;
extern crate reqwest;

mod djs;

use djs::download::download;
use djs::mediator::Mediator;
use djs::console::ConsoleMediator;
use std::io::{stderr, Write};
use std::process::{exit};
use djs::config::{Config, validate_config};
use djs::cli::{configure_from_cli, build_cli};
use djs::rc::{configure_from_file};
use djs::jenkins::Jenkins;
use djs::git::guess_branch;

fn main() {
    #![allow(unused_must_use)]
    env_logger::init();
    let cli = build_cli();
	let opts = cli.get_matches();
    let mut c = Config {..Default::default()};
    let mut mediator = ConsoleMediator::new();
    debug!("Start State, c = {:?}", c);


    // start from the default config
    // then 'guess' the git branch
    //   if it's specfiied in the file or local .rc file then we ignore the branch
    //
    if let Some(git_branch) = guess_branch() {
        debug!("Guessed git branch is {:?}", git_branch);
        c.branch = git_branch;
    }
    // read from file
    // override from command line

    configure_from_file(&mut c).expect("Failed to configure from the file.");
    debug!("About to configure from CLI");
    configure_from_cli(&mut c, &opts).expect("Failed to parse the CLI");


    if let Some(err) = validate_config(&c).err() {
        writeln!(stderr(), "{:?}", err).unwrap();
        exit(1)
    }

    let destination_path = c.destination_path().clone();
    let dry_run = c.dry_run;
    let mut j = Jenkins::new(&mut c);
    debug!("Jenkins = {:?}", j);

    let download_result = j.resolve_download_url()
        .and_then(|url| {
            mediator.print(format!("Resolved URL: {}", url));
            if !dry_run {
                download(url.as_str(), destination_path.as_str(), &mut mediator)
                    .map_err(|e| String::from(e.description()))
            } else {
                mediator.print("Dry Run, not downloading the file.");
                Ok(())
            }
        });

    match download_result {
        Err(err) => {
            writeln!(stderr(), "{:?}", err).unwrap();
            exit(1)
        },
        Ok(_) => {
            println!("Done");
            exit(0)
        }
    }
}

