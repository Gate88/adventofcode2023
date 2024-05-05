mod day01;
mod day02;
mod day03;
mod grouper;

fn main() {
    main_d03()
}

fn main_d03() {
    day03::part1()
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