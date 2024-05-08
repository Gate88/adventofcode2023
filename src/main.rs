use proc_macro_lib::include_all_day_files;
use std::{env::args, fmt::Display};

#[macro_use]
mod macros;
mod grouper;

include_all_day_files!("change to pull files");

fn main() {
    let s = args().nth(1).unwrap_or_else(|| get_default_day());
    println!("== Day {s} ==");
    run_day(s.parse().expect("day is not a number")).expect("could not find function for day");
}

trait Time<T>: Fn() -> T {
    fn time(&self, part: usize);
}

impl<T, U> Time<U> for T
where
    T: Fn() -> U,
    U: Display,
{
    fn time(&self, part: usize) {
        use std::time::Instant;
        let now = Instant::now();
        print!("PART {part}: ");
        println!("{}", self());
        println!(" took {:.2?}", now.elapsed());
    }
}
