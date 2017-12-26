
use clap::{Arg, App, ArgMatches};
use djs::config::Config;

#[allow(unused_imports)]
use djs::defaults::*;


macro_rules! set_config {
    ($config: ident, $opts:ident, $option: ident, $arg: expr) => {
        if let Some(v) = $opts.value_of(stringify!($option)) {
            debug!("  cli option {} = {}", stringify!($option), v);
            $config.$option.set(v.trim().to_string(), $arg.to_string());
        }
    }
}

pub fn configure_from_cli(c : &mut Config, opts: &ArgMatches) -> Result<(), String> {
    debug!("configure_from_cli");
    set_config!(c, opts, url, "-u");
    set_config!(c, opts, base, "-e");
    set_config!(c, opts, project, "-p");
    set_config!(c, opts, branch, "-b");
    set_config!(c, opts, build, "-j");
    set_config!(c, opts, solution, "-s");
    set_config!(c, opts, destination, "-d");

    if opts.is_present("dry_run") {
        c.dry_run.set(true, String::from("cli"));
    }
    if opts.is_present("verbose") {
        c.verbose.set(true, String::from("verbose"));
    }
    if opts.is_present("quiet") {
        c.quiet.set(true, String::from("cli"));
    }
    Ok(())
}

pub fn build_cli() -> App<'static, 'static> {
    let app = App::new("Jenkins Solution Downloader (jds)")
        .version(crate_version!())
        .author("Trever Shick <trever.shick@tanium.com>")
        .about("Helps download solution XMLs from Jenkins")
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .value_name("Jenkins URL")
            .takes_value(true))
        .arg(Arg::with_name("base")
            .short("e")
            .long("base")
            .value_name("Base URL before getting to project root")
            .takes_value(true))
        .arg(Arg::with_name("project")
            .short("p")
            .long("project")
            .value_name("Project Name (Jenkins Path Element)")
            .takes_value(true))
        .arg(Arg::with_name("branch")
            .short("b")
            .long("branch")
            .value_name("BRANCH")
            .takes_value(true))
        .arg(Arg::with_name("build")
            .short("j")
            .long("build")
            .value_name("BUILD NUMBER")
            .takes_value(true))
        .arg(Arg::with_name("solution")
            .short("s")
            .long("solution")
            .value_name("SOLUTION")
            .takes_value(true))
        .arg(Arg::with_name("destination")
            .short("d")
            .long("destination")
            .value_name("SOLUTION")
            .help("Sets the branch to download")
            .takes_value(true))
        .arg(Arg::with_name("verbose")
            .short("v")
            .help("If set to true, extra information will be sent to the console"))
        .arg(Arg::with_name("dry_run")
            .short("n")
            .help("If set to true, nothing will be downloaded."))
        .arg(Arg::with_name("quiet")
            .short("q")
            .multiple(false)
            .help("Turns off output"));
    return app;
}
