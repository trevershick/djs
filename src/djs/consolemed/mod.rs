use djs::mediator::Mediator;

extern crate console;
extern crate indicatif;

use console::{Term};
use console::style;
use djs::config::Config;
use self::indicatif::{HumanBytes, ProgressBar, ProgressStyle};
use std::rc::Rc;
use std::cell::RefCell;

pub struct ConsoleMediator {
    config: Rc<RefCell<Config>>,
    progress_bar: Option<ProgressBar>,
}

impl ConsoleMediator {
    pub fn new(config: Rc<RefCell<Config>>) -> ConsoleMediator {
        ConsoleMediator {
            config: config,
            progress_bar: None,
        }
    }
}

impl Mediator for ConsoleMediator {
    fn print(&self, out: String) {
        if !(*self.config.borrow()).quiet.get() {
            println!("{}", out);
        }
    }

    fn start_step(&mut self, step: &str) {
        if self.config.borrow().quiet.get() {
            return;
        }
        print!("{}...", step);
    }

    fn finish_step(&mut self) {
        if self.config.borrow().quiet.get() {
            return;
        }
        println!("{}", style("done").green());
    }

    fn start_progress(&mut self, name: &str, total_value: Option<u64>) {
        if self.config.borrow().quiet.get() {
            return;
        }
        if self.progress_bar.is_some() {
            panic!("The progress bar has already been setup!!!");
        }
        self.progress_bar = Some(create_progress_bar(name, total_value));
    }

    fn incr_progress(&mut self, _name: &str, incr_by: u64) {
        if self.config.borrow().quiet.get() {
            return;
        }
        if let Some(ref mut b) = self.progress_bar {
            b.inc(incr_by);
        }
    }

    fn finish_progress(&mut self, _name: &str) {
        if self.config.borrow().quiet.get() {
            return;
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
    let w = Term::stdout().size().1 as usize;
    let tot = if w < 100 { None } else { total };

    let bar = match tot {
        Some(v) => {
            let bar = ProgressBar::new(v);
            bar.set_style(ProgressStyle::default_bar()
                .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
                .progress_chars("=> "));
            bar.enable_steady_tick(250);
            bar
        }
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
