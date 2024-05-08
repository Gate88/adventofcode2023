use lazy_static::lazy_static;
use std::collections::HashSet;

use regex::Regex;

const INPUT_DAY04: &str = include_str!(r"..\input\day04.txt");

lazy_static! {
    static ref CARD_RE: Regex =
        Regex::new(r"^\s*Card\s*(?P<id>\d*):(?P<win>[^|]*)\|(?P<pick>[^|]*)$").unwrap();
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
}

struct Card {
    _id: usize,
    winning_numbers: HashSet<i32>,
    picked_numbers: HashSet<i32>,
}

impl Card {
    fn all_cards(input: &str) -> Vec<Card> {
        input.lines().map(|l| Card::new(l)).collect()
    }

    fn new(line: &str) -> Card {
        let captures = CARD_RE.captures(line).unwrap();
        Card {
            _id: captures
                .name("id")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap()
                - 1,
            winning_numbers: hashset_from_numbers(captures.name("win").unwrap().as_str()),
            picked_numbers: hashset_from_numbers(captures.name("pick").unwrap().as_str()),
        }
    }

    fn matches(&self) -> usize {
        self.picked_numbers
            .intersection(&self.winning_numbers)
            .count()
    }

    fn get_points(&self) -> i32 {
        let matches = self.matches();
        if matches == 0 {
            0
        } else {
            2i32.pow((matches - 1) as u32)
        }
    }
}

pub fn part1() {
    let p1: i32 = Card::all_cards(INPUT_DAY04)
        .into_iter()
        .map(|c| c.get_points())
        .sum();
    println!("part1: {p1}")
}

pub fn part2() {
    let cards = Card::all_cards(INPUT_DAY04);
    println!("part2: {}", chain_cards(&cards));
}

fn chain_cards(cards: &Vec<Card>) -> usize {
    let mut used: usize = 0;
    let mut to_process: Vec<usize> = vec![1; cards.len()];
    for i in 0..to_process.len() {
        let count = to_process[i];
        for j in 0..cards[i].matches() {
            to_process[i + 1 + j] += count
        }
        used += count
    }
    used
}

fn hashset_from_numbers(s: &str) -> HashSet<i32> {
    NUM_RE
        .find_iter(s)
        .map(|s| s.as_str().parse::<i32>().unwrap())
        .collect()
}
