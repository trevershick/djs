use djs::config::{Config};
//pub fn locate_djsrc() -> Option<String> {
//    None
//}
//
//pub fn read_djsrc(path: String) -> Result<Config, String> {
//    // read the file specified by path or panic
//    // populate a new Config object with the values from the file
//    Ok(Config{ ..Default::default() })
//}

pub fn configure_from_file(config: &Config) -> Result<&Config, String> {
    // load the file A
        // if it exists,
        //  read it in
        //  update the config
        // if it doesn't just return
    Ok(config)
}

// Initializes an .jdsrc file in the current directory.
//
// If the file was created then the path is returned
// If the file wasn't created then an error is returned.
//fn create_djsrc_file(config: &Config) -> Result<(),String> {
//    // build the file from the current configuration
//    Ok(())
//}
