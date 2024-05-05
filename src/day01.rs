use regex::Regex;

const DAY01_INPUT: &str = include_str!(r"..\input\day01.txt");

pub fn part1() {
    let input = DAY01_INPUT;
    let re = Regex::new(r"^[^\d]*(\d).*(\d)[^\d]*$|^[^\d]*(\d)[^\d]*$").unwrap();
    let p1: i32 = input.lines().map(|line| {
        let captures = re.captures(line).unwrap();

        let a = match captures.get(1) {
            Some(m) => m,
            None => captures.get(3).unwrap(),
        }.as_str();

        let b = match captures.get(2) {
            Some(m) => m,
            None => captures.get(3).unwrap(),
        }.as_str();

        format!("{a}{b}").parse::<i32>().unwrap()
    }).sum();

    println!("part1: {p1}");
}

pub fn part2() {
    let input = DAY01_INPUT;
    let n_strings = r"one|two|three|four|five|six|seven|eight|nine";
    let rev_n_strings = rev(n_strings);
    let reg = Regex::new(&format!("\\d|{n_strings}")).unwrap();
    let reg_rev = Regex::new(&format!("\\d|{rev_n_strings}")).unwrap();

    let p2: i32 = input.lines().map(|line| {
        let first = reg.find(line).unwrap().as_str();
        let line_reverse = rev(line);
        let last = reg_rev.find(&line_reverse).unwrap().as_str();

        let first = match_to_digit(first);
        let last_rev = rev(last);
        let last = match_to_digit(&last_rev);

        format!("{first}{last}").parse::<i32>().unwrap()
    }).sum();

    print!("part2: {p2}");
}

fn rev(s: &str) -> String {
    return s.chars().rev().collect::<String>();
}

fn match_to_digit(s: &str) -> &str {
    match s {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        s => s,
    }
}