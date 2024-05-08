use std::{cmp::Ordering, collections::HashMap};

const _DAY07_SIMPLE_INPUT: &str = include_str!(r"..\input\day07_simple.txt");
const DAY07_INPUT: &str = include_str!(r"..\input\day07.txt");

const JOKER: char = 'J';

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

#[derive(Debug)]
struct Hand {
    cards: [char; 5],
    bid: usize,
    joker_mode: bool,
}

impl Hand {
    fn new(line: &str, joker_mode: bool) -> Self {
        let mut items = line.split(" ");

        let cards = items
            .next()
            .expect("could not find cards")
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .expect("wrong number of cards");

        let bid = items
            .next()
            .expect("could not find bid")
            .parse()
            .expect("could not parse bid");

        Hand {
            cards,
            bid,
            joker_mode,
        }
    }

    fn get_hand_type(&self) -> HandType {
        self.get_hand_type_with_joker_mode(self.joker_mode)
    }

    //Group<card character, card count>
    fn get_card_type_groups(&self) -> HashMap<&char, u32> {
        self.cards.iter().fold(HashMap::new(), |mut hashmap, c| {
            *hashmap.entry(c).or_insert_with(|| 0) += 1;
            return hashmap;
        })
    }

    fn get_hand_type_with_joker_mode(&self, joker_mode: bool) -> HandType {
        use HandType::*;

        if joker_mode {
            return self.get_hand_type_with_joker_mode_on();
        }

        let card_type_groups = self.get_card_type_groups();
        match card_type_groups.len() {
            1 => FiveOfAKind,
            2 => card_type_groups
                .values()
                .next()
                .map(|count| match count {
                    1 | 4 => FourOfAKind,
                    _ => FullHouse,
                })
                .unwrap(),
            3 => card_type_groups
                .values()
                .find_map(|&count| match count {
                    3 => Some(ThreeOfAKind),
                    2 => Some(TwoPair),
                    _ => None,
                })
                .unwrap(),
            4 => OnePair,
            _ => HighCard,
        }
    }

    fn get_hand_type_with_joker_mode_on(&self) -> HandType {
        use HandType::*;

        //HashMap<card, card_count>
        let card_type_groups = self.get_card_type_groups();
        let joker_count = *card_type_groups.get(&JOKER).unwrap_or(&0);
        let non_joker_groups: HashMap<&char, u32> = card_type_groups
            .into_iter()
            .filter(|(&card, _)| card != JOKER)
            .collect();

        match joker_count {
            0 => self.get_hand_type_with_joker_mode(false),
            _ => match non_joker_groups.len() {
                0 | 1 => FiveOfAKind,
                2 => non_joker_groups
                    .values()
                    .find_map(|count| (count + joker_count == 4).then_some(FourOfAKind))
                    .unwrap_or(FullHouse),
                3 => ThreeOfAKind,
                _ => OnePair,
            },
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn card_strength(c: &char, joker_mode: bool) -> u32 {
    match c {
        &JOKER if joker_mode => 1,
        '2'..='9' => (*c as u32) - ('2' as u32) + 2,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("unknown card: {c}"),
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        assert!(
            self.joker_mode == other.joker_mode,
            "hands must be in same joker mode to be compared"
        );

        match self.get_hand_type().cmp(&other.get_hand_type()) {
            o @ (Greater | Less) => o,
            Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(self_card, other_card)| {
                    //If there are unequal strengths, then that's the result
                    Some(
                        card_strength(&self_card, self.joker_mode)
                            .cmp(&card_strength(&other_card, other.joker_mode)),
                    )
                    .filter(|&s| s != Equal)
                })
                //If we didn't find unequal strengths, then the whole thing is equal
                .unwrap_or(Equal),
        }
    }
}

fn get_hands(input: &str, joker_mode: bool) -> Vec<Hand> {
    input.lines().map(|l| Hand::new(l, joker_mode)).collect()
}

fn sort_and_calc_winnings(mut hands: Vec<Hand>) -> (usize, Vec<Hand>) {
    hands.sort_unstable();
    (
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i + 1))
            .sum(),
        hands,
    )
}

pub fn part1() {
    let hands = get_hands(DAY07_INPUT, false);
    let (p1, _) = sort_and_calc_winnings(hands);
    println!("part1: {p1}")
}

pub fn part2() {
    let hands = get_hands(DAY07_INPUT, true);
    let (p2, _) = sort_and_calc_winnings(hands);
    println!("part1: {p2}")
}
