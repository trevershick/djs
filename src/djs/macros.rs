#[macro_export]
macro_rules! dump_configm {
    ($mediator:ident, $config: ident, $title:expr, $opt:ident) => {
        let value = match $config.$opt().len() {
            0 => "<empty>".to_string(),
            _ => $config.$opt()
        };
        $mediator.print(format!("{} ({}): {}",
                               $title,
                               stringify!($opt),
                               style(value).green(),
                               ));
    }
}

macro_rules! dump_config {
    ($mediator:ident, $config: ident, $title:expr, $opt:ident) => {
        let value = match $config.$opt.get().len() {
            0 => "<empty>".to_string(),
            _ => $config.$opt.get()
        };
        $mediator.print(format!("{} ({}): {} [source: {}]",
                               $title,
                               stringify!($opt),
                               style(value).green(),
                               style($config.$opt.source()).magenta(),
                               ));
    }
}
