use std::env::args;

#[macro_use]
mod day_macros;
mod grouper;

main_day!(
    run_day,
    get_default_day,
    day01,
    day02,
    day03,
    day04,
    day05,
    day06,
    day07,
    day08
);

fn main() {
    let s = args().nth(1).unwrap_or(get_default_day());
    println!("== Day {s} ==");
    run_day(s.parse().expect("day is not a number")).expect("could not find function for day");
}

trait Time: Fn() -> String {
    fn time(&self, part: usize);
}

impl<T> Time for T
where
    T: Fn() -> String,
{
    fn time(&self, part: usize) {
        use std::time::Instant;
        let now = Instant::now();
        println!("PART {part}: {}", self());
        println!(" took {:.2?}", now.elapsed());
    }
}
