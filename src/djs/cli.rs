
use clap::{Arg, App, ArgMatches};
use djs::config::Config;

#[allow(unused_imports)]
use djs::defaults::*;


macro_rules! set_config {
    ($config: ident, $opts:ident, $option: ident) => {
        if let Some(v) = $opts.value_of(stringify!($option)) {
            debug!(" cli option {} = {}", stringify!($option), v);
            $config.$option = String::from(v);
        }
    }
}

pub fn configure_from_cli(c : &mut Config, opts: &ArgMatches) -> Result<(), String> {
    debug!("Configuring from the CLI");
    set_config!(c, opts, url);
    set_config!(c, opts, base);
    set_config!(c, opts, project);
    set_config!(c, opts, branch);
    set_config!(c, opts, build);
    set_config!(c, opts, solution);
    set_config!(c, opts, destination);

    if opts.is_present("dry_run") {
        c.dry_run = true;
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
            .default_value(DEFAULT_URL)
            .takes_value(true))
        .arg(Arg::with_name("base")
            .short("e")
            .long("base")
            .value_name("Base URL before getting to project root")
            .default_value(DEFAULT_BASE)
            .takes_value(true))
        .arg(Arg::with_name("project")
            .short("p")
            .long("project")
            .value_name("Project Name (Jenkins Path Element)")
            .default_value(DEFAULT_PROJECT)
            .takes_value(true))
        .arg(Arg::with_name("branch")
            .short("b")
            .long("branch")
            .value_name("BRANCH")
            .default_value(DEFAULT_BRANCH)
            .takes_value(true))
        .arg(Arg::with_name("build")
            .short("j")
            .long("build")
            .value_name("BUILD NUMBER")
            .default_value(DEFAULT_BUILD)
            .takes_value(true))
        .arg(Arg::with_name("solution")
            .short("s")
            .long("solution")
            .value_name("SOLUTION")
            .default_value(DEFAULT_SOLUTION)
            .takes_value(true))
        .arg(Arg::with_name("destination")
            .short("d")
            .long("destination")
            .value_name("SOLUTION")
            .default_value(DEFAULT_SOLUTION)
            .help("Sets the branch to download")
            .takes_value(true))
        .arg(Arg::with_name("dry_run")
            .short("n")
            .help("If set to true, nothing will be downloaded."))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"));
    return app;
}
