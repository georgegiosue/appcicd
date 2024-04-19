use std::sync::Mutex;
use std::{fmt::Display, process::exit};

pub fn print_out<T: Display>(msg: T) {
    println!("{}", msg);
}

pub fn panic_out<T: Display>(msg: T) {
    println!("{}", msg);
    exit(0)
}

pub static VERBOSE: Mutex<bool> = Mutex::new(false);

pub fn set_verbose(value: bool) {
    if let Ok(mut guard) = VERBOSE.lock() {
        *guard = value;
    }
}

#[macro_export]
macro_rules! verbose_println {
    ($($arg:tt)*) => {
        if let Ok(guard) = crate::utils::out::VERBOSE.lock() {
            if *guard {
                println!($($arg)*);
            }
        }
    };
}
