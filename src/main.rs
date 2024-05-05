use std::env::args;

mod day01;
mod day02;
mod day03;
mod grouper;

fn main() {
    let s = args().nth(1).unwrap_or("03".to_owned());
    println!("Day{s}:");
    match s.as_str() {
        "01" => main_d01(),
        "02" => main_d02(),
        "03" => main_d03(),
        _ => panic!("not a valid day")
    };
}

fn main_d03() {
    day03::part1();
    day03::part2();
}

#[allow(dead_code)]
fn main_d02() {
    let p1 = day02::part1();
    println!("part1: {p1}");
    let p2 = day02::part2();
    println!("part2: {p2}");
}

#[allow(dead_code)]
fn main_d01() {
    let p1 = day01::part1();
    println!("part1: {p1}");
    let p2 = day01::part2();
    print!("part2: {p2}");
}