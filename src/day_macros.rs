macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

macro_rules! main_day {
    ( $run_day:ident, $get_default_day:ident, $($day:ident),* ) => {
        $(
            mod $day;
        )*

        fn $run_day(day: usize) -> Option<()>{
            let mut count = 0usize;
            $(
                count += 1usize;
                if count == day {
                    use $day::*;
                    println!();
                    part1.time(1);
                    println!();
                    part2.time(2);
                    println!();
                    return Some(())
                }
            )*

            return None
        }

        fn $get_default_day() -> String {
            let mut count = 0usize;
            $(
                count += replace_expr!($day 1usize);
            )*
            format!("{count}")
        }
    };
}
