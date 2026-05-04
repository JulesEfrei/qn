macro_rules! log {
    ($level:expr, $message:expr) => {
        if cfg!(debug_assertions) {
            println!("[{}] {}", $level, $message);
        }
    };

    ($level:expr, $message:expr, $file:expr, $line:expr) => {
        if cfg!(debug_assertions) {
            println!("[{}] {} (at {}:{})", $level, $message, $file, $line);
        }
    };
}

pub(crate) use log;
