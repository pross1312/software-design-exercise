#[macro_export] macro_rules! log {
    ($($arg:tt)*) => { std::fs::File::options().create(true).append(true).open("data.log").unwrap().write_all(
            format!("{} : {}\n", chrono::Local::now(), (format!($($arg)*))).as_bytes()
        ).unwrap()
    };
}
