use std::env::args;

#[macro_use]
mod macros;
mod days;
mod helper;

fn main() {
    let s = args().nth(1).unwrap_or_else(|| days::get_default_day());
    println!("== Day {s} ==");
    days::run_day(s.parse().expect("day is not a number"))
        .expect("could not find function for day");
}
