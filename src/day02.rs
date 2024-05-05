use std::collections::HashMap;
use std::hash::Hash;

use regex::Regex;


const DAY01_INPUT: &'static str = include_str!(r"..\input\day02.txt");

#[derive(PartialEq, Eq)]
#[derive(Hash)]
#[derive(Debug)] 
enum Color {
    Red,
    Green,
    Blue,
}

struct Game {
    id: i64,
    turns: Vec<Turn>,
}

struct Turn {
    picks: Vec<Pick>,
}

struct Pick {
    count: i64,
    color: Color,
}

fn get_games(input: &str) -> Vec<Game> {
    let game_reg = Regex::new(r"Game (?P<id>\d*): (?P<turns>.*)").unwrap();
    let turn_reg = Regex::new(r"[^;]*").unwrap();
    let pick_reg = Regex::new(r"(?P<count>\d*) (?P<color>red|green|blue)").unwrap();

    input.lines().map(|line| {
        let captures = game_reg.captures(line).unwrap();
        let turns = captures.name("turns").unwrap().as_str();

        Game {
            id: captures.name("id").unwrap().as_str().parse::<i64>().unwrap(),
            turns: turn_reg.find_iter(turns).map(|turn| {
                Turn {
                    picks: pick_reg.captures_iter(turn.as_str()).map(|pick| {
                        Pick {
                            count: pick.name("count").unwrap().as_str().parse::<i64>().unwrap(),
                            color: match pick.name("color").unwrap().as_str() {
                                "red" => Color::Red,
                                "blue" => Color::Blue,
                                "green" => Color::Green,
                                c => panic!("cannot parse color {c}")
                            }
                        }
                    }).collect(),
                }
            }).collect(),
        }
    }).collect()
}

pub fn part1() -> i64 {
    get_games(DAY01_INPUT).iter().map(|g| {
        for pick in g.turns.iter().map(|t| &t.picks).flatten() {
            match pick.color {
                Color::Red if pick.count > 12 => return 0,
                Color::Green if pick.count > 13 => return 0,
                Color::Blue if pick.count > 14 => return 0,
                _ => {},
            }
        }
        return g.id;
    }).sum()
}

pub fn part2() -> i64 {
    get_games(DAY01_INPUT).iter().map(|g| {
        let groups = group_by(g.turns.iter().map(|t| &t.picks).flatten(),|p| &p.color);
        groups.into_iter().map(|g| {
            g.1.into_iter().map(|p| p.count).max().unwrap()
        }).product::<i64>()
    }).sum()
}

fn group_by<K, V, I, F>(i: I, mut f: F) -> HashMap<K, Vec<V>>
where
    K: Eq + Hash,
    I: IntoIterator<Item = V> + Sized,
    F: FnMut(&I::Item) -> K
{
    let mut result = HashMap::<K, Vec<V>>::new();
    for item in i {
        let key = f(&item);
        if result.get(&key).is_none() {
            result.insert(key, vec![item]);
        } else {
            result.get_mut(&key).unwrap().push(item)
        }
    }

    result
}