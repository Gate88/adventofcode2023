use std::env::args;

mod day01;
mod day02;
mod day03;
mod day04;
mod grouper;

fn main() {
    let s = args().nth(1).unwrap_or("4".to_owned());
    println!("== Day{s} ==");
    match s.as_str() {
        "1" => main_d01(),
        "2" => main_d02(),
        "3" => main_d03(),
        "4" => main_d04(),
        _ => panic!("not a valid day")
    };
}

fn main_d04() {
    day04::part1();
    day04::part2();
}

fn main_d03() {
    day03::part1();
    day03::part2();
}

fn main_d02() {
    day02::part1();
    day02::part2();}

fn main_d01() {
    day01::part1();
    day01::part2();
}