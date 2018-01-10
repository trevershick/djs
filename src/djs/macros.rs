#[macro_export]

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

macro_rules! s {
	($str_ref:expr) => { $str_ref.to_string() }
}

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
        let v : String = format!("{}", $config.$opt.get());
        let value = match v.len() {
            0 => "<empty>".to_string(),
            _ => v
        };
        $mediator.print(format!("{} ({}): {} [source: {}]",
                               $title,
                               stringify!($opt),
                               style(value).green(),
                               style($config.$opt.source()).magenta(),
                               ));
    }
}
