extern crate configuration;

use djs::config::Config;
use std;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;
use self::configuration::format::TOML;

macro_rules! set_config {
    ($config: ident, $tree: ident, $option: ident, $source: ident) => {
        if let Some(v) = $tree.get::<String>(stringify!($option)) {
            debug!("  tree option {} = {}", stringify!($option), v);
            $config.borrow_mut().$option.set(v.clone(), $source.to_string());
        }
    }
}

pub fn configure_from_file(
    p: &Path,
    config: Rc<RefCell<Config>>,
) -> Result<(), Box<std::error::Error>> {
    debug!("configure_from_file, p={:?}", p);
    if let Ok(tree) = TOML::open(p) {
        debug!("  tree loaded");
        let p_str = p.to_str().unwrap_or("?");
        set_config!(config, tree, url, p_str);
        set_config!(config, tree, base, p_str);
        set_config!(config, tree, project, p_str);
        set_config!(config, tree, branch, p_str);
        set_config!(config, tree, build, p_str);
        set_config!(config, tree, solution, p_str);
        set_config!(config, tree, solution_filter, p_str);
        set_config!(config, tree, destination, p_str);

        if let Some(v) = tree.get::<i32>("timeout") {
            config.borrow_mut().timeout_in_seconds.set(*v, p_str);
        }
    }

    // load the file A
    // if it exists,
    //  read it in
    //  update the config
    // if it doesn't just return
    Ok(())
}

// Initializes an .jdsrc file in the current directory.
//
// If the file was created then the path is returned
// If the file wasn't created then an error is returned.
//fn create_djsrc_file(config: &Config) -> Result<(),String> {
//    // build the file from the current configuration
//    Ok(())
//}
