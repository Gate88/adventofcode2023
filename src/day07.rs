use std::{cmp::Ordering, collections::HashMap};

use crate::grouper::Grouper;

const _DAY07_SIMPLE_INPUT: &str = include_str!(r"..\input\day07_simple.txt");
const DAY07_INPUT: &str = include_str!(r"..\input\day07.txt");

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

struct Hand<'a> {
    cards: &'a str,
    bid: usize,
    joker_mode: bool,
}

fn strength(c: &char, joker_mode: bool) -> u8 {
    match c {
        '2'..='9' => (*c as u8) - ('2' as u8) + 2,
        'T' => 10,
        'J' => if joker_mode {1} else {11},
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("unknown card: {}", c)
    }
}

impl<'a> Hand<'a> {
    fn new(line: &'a str, joker_mode: bool) -> Option<Self> {
        let mut items = line.split(" ");
        let cards = items.next()?;
        assert!(cards.chars().count() == 5, "hand must have exactly 5 cards");
        let bid = items.next()?.parse().ok()?;
        Some(Hand {
            cards,
            bid,
            joker_mode,
        })
    }

    fn get_hand_type(&self) -> HandType {
        self._get_hand_type(self.joker_mode)
    }

    fn _get_hand_type(&self, joker_mode: bool) -> HandType {
        if joker_mode { return self.get_hand_type_joker_mode() }
        
        let card_type_groups = self.cards.chars().group_by(|i| i.clone());
        match card_type_groups.keys().len() {
            1 => HandType::FiveOfAKind,
            2 => {
                match card_type_groups.values().nth(0).unwrap().len() {
                    1 | 4 => HandType::FourOfAKind,
                    _ => HandType::FullHouse,
                }
            },
            3 => {
                for v in card_type_groups.values() {
                    if v.len() == 3 { return HandType::ThreeOfAKind }
                    if v.len() == 2 { return HandType::TwoPair }
                }
                panic!("impossible state");
            },
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn get_hand_type_joker_mode(&self) -> HandType {
        let card_type_groups = self.cards.chars().group_by(|i| i.clone());
        let joker_count = card_type_groups.get(&'J').map_or(0, |vec| vec.len());
        let non_joker_groups: HashMap<char, Vec<char>> = card_type_groups.into_iter().filter(|i| i.0 != 'J').collect();

        match joker_count {
            0 => self._get_hand_type(false),
            _ => match non_joker_groups.keys().len() {
                0 => HandType::FiveOfAKind,
                1 => HandType::FiveOfAKind,
                2 => {
                    for v in non_joker_groups.values() {
                        if v.len() + joker_count == 4 { return HandType::FourOfAKind }
                    }
                    HandType::FullHouse
                },
                3 => HandType::ThreeOfAKind,
                _ => HandType::OnePair,
            }
        }
    }

}

impl <'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl <'a> Eq for Hand<'a> {}

impl <'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        assert!(self.joker_mode == other.joker_mode, "hands must be in same joker mode to be compared");
        match self.get_hand_type().cmp(&other.get_hand_type()) {
            o @ (Ordering::Greater | Ordering::Less) => o,
            Ordering::Equal => {
                let s = self.cards.chars();
                let o = other.cards.chars();
                for (s, o) in s.zip(o) {
                    let cmp = strength(&s, self.joker_mode).cmp(&strength(&o, other.joker_mode));
                    if cmp != Ordering::Equal { return cmp }
                }
                Ordering::Equal
            }
        }
    }
}

fn get_hands(input: &str, joker_mode: bool) -> Vec<Hand> {
    input.lines().filter_map(|l| Hand::new(l, joker_mode)).collect()
}

pub fn part1() {
    let mut hands = get_hands(DAY07_INPUT, false);
    hands.sort_unstable();
    let p1: usize = hands.iter().enumerate().map(|(i, h)| {
        h.bid * (i+1)
    }).sum();
    println!("part1: {p1}")
}

pub fn part2() {
    let mut hands = get_hands(DAY07_INPUT, true);
    hands.sort_unstable();
    let p2: usize = hands.iter().enumerate().map(|(i, h)| {
        h.bid * (i+1)
    }).sum();
    println!("part1: {p2}")
}