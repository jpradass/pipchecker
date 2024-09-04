#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        let level = "info".blue().bold();
        println!("{}: {}", level, $msg);
    };
}

macro_rules! warn {
    ($msg:expr) => {
        let level = "warn".yellow().bold();
        println!("{}: {}", level, $msg);
    };
}

macro_rules! error {
    ($msg:expr) => {
        let level = "error".red().bold();
        println!("{}: {}", level, $msg);
    };
}
