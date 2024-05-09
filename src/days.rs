use std::fmt::Display;

use proc_macro_lib::include_all_day_files;

include_all_day_files!("change to pull files");
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
