
/*
 * file for macros like info and error
 */

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        println!("{} {}", "INFO:".green().bold(), format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        println!("{} {}", "WARN:".yellow().bold(), format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        println!("{} {}", "ERROR:".red().bold(), format!($($arg)*));
        std::process::exit(1);
    }};
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        println!("{} {}", "SUCCESS:".green().bold(), format!($($arg)*));
    }};
}
