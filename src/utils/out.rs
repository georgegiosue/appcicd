use std::{fmt::Display, process::exit};

pub fn print_out<T: Display>(msg: T) {
    println!("{}", msg);
}

pub fn panic_out<T: Display>(msg: T) {
    println!("{}", msg);
    exit(0)
}
