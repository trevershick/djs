#[allow(unused_imports)]
extern crate console;
#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate clap;
extern crate reqwest;

mod djs;

use console::{style};
use std::env::home_dir;
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
use std::path::Path;

macro_rules! dump_config {
    ($mediator:ident, $config: ident, $title:expr, $opt:ident) => {
       $mediator.print(format!("{} ({}): {} [source: {}]",
                               stringify!($title),
                               stringify!($opt),
                               style($config.$opt.get()).green(),
                               style($config.$opt.source()).magenta(),
                               ));
    }
}

fn main() {
    #![allow(unused_must_use)]
    env_logger::init();
    let cli = build_cli();
	let opts = cli.get_matches();
    let mut c = Config {..Default::default()};

    debug!("initial config={:?}", c);

    if let Some(mut home_pb) = home_dir() {
        home_pb.push(".djsrc");
        configure_from_file(home_pb.as_path(), &mut c);
    }
    configure_from_file(Path::new("./.djsrc"), &mut c);

    // start from the default config
    // then 'guess' the git branch
    //   if it's specfiied in the file or local .rc file then we ignore the branch
    //
    if let Some(git_branch) = guess_branch() {
        debug!("Guessed git branch is {:?}", git_branch);
        c.branch.set(git_branch, String::from("git"));
    }
    // read from file
    // override from command line


    debug!("About to configure from CLI");
    configure_from_cli(&mut c, &opts).expect("Failed to parse the CLI");


    if let Some(err) = validate_config(&c).err() {
        writeln!(stderr(), "{:?}", err).unwrap();
        exit(1)
    }

    let destination_path = c.destination_path().clone();
    let dry_run = c.dry_run.get();
    let verbose = c.verbose.get();

    // i don't like this.  the mediator only needs to read from the config
    // while the jenkins struct needs to modify it
    let config_snapshot = c.clone();
    let mut mediator = ConsoleMediator::new(&config_snapshot);

    let mut j = Jenkins::new(&mut c);
    debug!("Jenkins = {:?}", j);

    if verbose {
       dump_config!(mediator, config_snapshot,"Jenkins Base URL", url);
       dump_config!(mediator, config_snapshot,"Jenkins Base Path", base);
       dump_config!(mediator, config_snapshot,"Project", project);
       dump_config!(mediator, config_snapshot,"Branch", branch);
       dump_config!(mediator, config_snapshot,"Build", build);
       dump_config!(mediator, config_snapshot,"Solution", solution);
       dump_config!(mediator, config_snapshot,"Destination", destination);
    }

    let download_result = j.resolve_download_url()
        .and_then(|url| {
            mediator.print(format!("Resolved URL: {}", style(url.as_str()).green()));
            if !dry_run {
                download(url.as_str(), destination_path.as_str(), &mut mediator)
                    .map_err(|e| String::from(e.description()))
            } else {
                mediator.print(format!("Dry Run, not downloading the file."));
                Ok(())
            }
        });

    match download_result {
        Err(err) => {
            writeln!(stderr(), "{} {}",style("ERROR").bold().red(), style(err).white());
            exit(1)
        },
        Ok(_) => {
            mediator.print(String::from("Done"));
            exit(0)
        }
    }
}

