use djs::mediator::Mediator;

extern crate indicatif;
extern crate console;

use djs::config::Config;
use self::indicatif::{ProgressBar, ProgressStyle, HumanBytes};
use std::rc::Rc;
use std::cell::RefCell;

pub struct ConsoleMediator {
    config: Rc<RefCell<Config>>,
    progress_bar : Option<ProgressBar>
}

impl ConsoleMediator {
    pub fn new(config: Rc<RefCell<Config>>) -> ConsoleMediator {
        ConsoleMediator { config: config, progress_bar: None }
    }
}


impl Mediator for ConsoleMediator {
    fn print(&self, out: String) {
        if !(*self.config.borrow()).quiet.get() {
            println!("{}", out);
        }
    }
    fn start_progress(&mut self, name: &str, total_value: Option<u64>) {
        if self.config.borrow().quiet.get() {
            return
        }
        if self.progress_bar.is_some() {
            panic!("The progress bar has already been setup!!!");
        }
        self.progress_bar = Some(create_progress_bar(name, total_value));
    }

    fn incr_progress(&mut self, _name: &str, incr_by: u64) {
        if self.config.borrow().quiet.get() {
            return
        }
        if let Some(ref mut b) = self.progress_bar {
            b.inc(incr_by);
        }
    }

    fn finish_progress(&mut self, _name: &str) {
        if self.config.borrow().quiet.get() {
            return
        }
        if let Some(ref mut b) = self.progress_bar {
            b.finish();
        }
        self.progress_bar = None;
    }

    fn human_bytes(&self, bytes: u64) -> String {
        format!("{}", HumanBytes(bytes))
    }
}


fn create_progress_bar(msg: &str, total: Option<u64>) -> ProgressBar {
    let bar = match total {
        Some(v) => {
            let bar = ProgressBar::new(v);
            bar.set_style(ProgressStyle::default_bar()
                .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
                .progress_chars("=> "));
            bar.enable_steady_tick(250);
            bar
        },
        None => {
            let bar = ProgressBar::new_spinner();
            bar.set_style(ProgressStyle::default_spinner());
            bar.enable_steady_tick(250);
            bar
        }
    };

    bar.set_message(msg);
    bar
}
            //bar.inc(bcount as u64);

        //let bar = create_progress_bar(fname, ct_len);
