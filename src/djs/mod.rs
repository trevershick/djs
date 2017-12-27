#[macro_use]
pub mod macros;

pub mod cli;
pub mod config;
pub mod rc;
pub mod jenkins;
pub mod mediator;
pub mod consolemed;
pub mod download;
pub mod git;
mod defaults;

use std::error::Error;
use console::style;

#[derive(Debug)]
pub enum DjsError {
    // option, current value, notes
    InvalidConfig(String, String, String)
}

use std::fmt;

impl fmt::Display for DjsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DjsError::InvalidConfig(ref opt, ref cur_val, ref notes) =>
                write!(f, "{opt} \"{cur_val}\" is invalid. Hint: {notes}",
                       opt = style(opt).green(),
                       cur_val = style(cur_val).red(),
                       notes = style(notes))
        }
    }
}

impl Error for DjsError {
    fn description(&self) -> &str {
        match *self {
            DjsError::InvalidConfig(_,_,_) => "Invalid Config"
        }
    }
}

