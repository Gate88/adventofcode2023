use std::env::args;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod grouper;

fn main() {
    let s = args().nth(1).unwrap_or("7".to_owned());
    println!("== Day{s} ==");
    match s.as_str() {
        "1" => main_d01(),
        "2" => main_d02(),
        "3" => main_d03(),
        "4" => main_d04(),
        "5" => main_d05(),
        "6" => main_d06(),
        "7" => main_d07(),
        _ => panic!("not a valid day"),
    };
}

fn main_d07() {
    day07::part1();
    day07::part2();
}

fn main_d06() {
    day06::part1();
    day06::part2();
}

fn main_d05() {
    day05::part1();
    day05::part2();
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
    day02::part2();
}

fn main_d01() {
    day01::part1();
    day01::part2();
}
