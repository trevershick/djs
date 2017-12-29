extern crate console;
extern crate env_logger;
extern crate reqwest;
#[macro_use] extern crate clap;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] mod djs;

use std::rc::Rc;
use std::path::Path;
use std::cell::RefCell;
use std::env::home_dir;
use std::io::{stderr, Write};
use std::process::exit;

use console::style;
use djs::download::download;
use djs::mediator::Mediator;
use djs::consolemed::ConsoleMediator;
use djs::config::{validate_config, Config};
use djs::cli::{build_cli, configure_from_cli};
use djs::rc::configure_from_file;
use djs::jenkins::Jenkins;
use djs::git::{guess_branch, guess_project};

fn main() {
    #![allow(unused_must_use)]
    env_logger::init();
    let cli = build_cli();
    let opts = cli.get_matches();

    let config = Rc::new(RefCell::new(Config {
        ..Default::default()
    }));

    if opts.is_present("quiet") {
        config.borrow_mut().quiet.set(true, "cli");
    }

    // i don't like this.  the mediator only needs to read from the config
    // while the jenkins struct needs to modify it
    let mut mediator = ConsoleMediator::new(Rc::clone(&config));

    debug!("initial config={:?}", config.borrow());

    mediator.start_step("Reading ~/.djsrc");
    if let Some(mut home_pb) = home_dir() {
        home_pb.push(".djsrc");
        configure_from_file(home_pb.as_path(), Rc::clone(&config));
    }
    mediator.finish_step();

    mediator.start_step("Reading .djsrc");
    configure_from_file(Path::new("./.djsrc"), Rc::clone(&config));
    mediator.finish_step();

    // start from the default config
    // then 'guess' the git branch
    //   if it's specfiied in the file or local .rc file then we ignore the branch
    mediator.start_step("Determine current git branch");
    if let Some(git_branch) = guess_branch() {
        debug!("Guessed git branch is {:?}", git_branch);
        // only override the value with the 'guess' if the branch value is
        // coming from defaults, not if it's from a file or command line
        config
            .borrow_mut()
            .branch
            .set_if_source(git_branch, "git", "defaults");
    }
    mediator.finish_step();

    mediator.start_step("Determine the current project");
    if let Some(git_project) = guess_project() {
        debug!("Guessed git project is {:?}", git_project);
        // only override the value with the 'guess' if the project value is
        // coming from defaults, not if it's from a file or command line
        config
            .borrow_mut()
            .project
            .set_if_source(git_project, "git", "defaults");
    }
    mediator.finish_step();

    debug!("About to configure from CLI");
    configure_from_cli(Rc::clone(&config), &opts).expect("Failed to parse the CLI");

    if let Some(err) = validate_config(Rc::clone(&config)).err() {
        writeln!(stderr(), "Error: {}", err).unwrap();
        exit(1)
    }

    let mut j = Jenkins::new(Rc::clone(&config));
    debug!("Jenkins = {:?}", j);

    mediator.start_progress("Resolving the download URL", None);
    let resolved_url = j.resolve_download_url();
    mediator.finish_progress("");
    if config.borrow().verbose.get() {
        let config_snapshot = config.borrow();
        dump_config!(mediator, config_snapshot, "Jenkins Base URL", url);
        dump_config!(mediator, config_snapshot, "Jenkins Base Path", base);
        dump_config!(mediator, config_snapshot, "Project", project);
        dump_config!(mediator, config_snapshot, "Branch", branch);
        dump_config!(mediator, config_snapshot, "Build", build);
        dump_config!(mediator, config_snapshot, "Solution", solution);
        dump_config!(
            mediator,
            config_snapshot,
            "Solution Filter",
            solution_filter
        );
        dump_config!(mediator, config_snapshot, "Destination", destination);
        dump_configm!(
            mediator,
            config_snapshot,
            "Destination Path",
            destination_path
        );
    }

    let download_result = resolved_url.and_then(|url| {
        mediator.print(format!("Resolved URL: {}", style(url.as_str()).green()));
        if !config.borrow().dry_run.get() {
            let destination_path = config.borrow().destination_path();

            download(url.as_str(), destination_path.as_str(), &mut mediator)
        } else {
            mediator.print(format!("Dry Run, not downloading the file."));
            Ok(())
        }
    });

    match download_result {
        Err(err) => {
            writeln!(
                stderr(),
                "{} {}",
                style("ERROR").bold().red(),
                style(err).white()
            );
            exit(1)
        }
        Ok(_) => {
            mediator.print(String::from("Done"));
            exit(0)
        }
    }
}
