use std::hash::Hash;
use regex::Regex;
use crate::grouper::Grouper;


const DAY02_INPUT: &str = include_str!(r"..\input\day02.txt");

#[derive(PartialEq, Eq, Hash)]
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

pub fn part1() {
    let p1: i64 = get_games(DAY02_INPUT).iter().map(|g| {
        for pick in g.turns.iter().map(|t| &t.picks).flatten() {
            match pick.color {
                Color::Red if pick.count > 12 => return 0,
                Color::Green if pick.count > 13 => return 0,
                Color::Blue if pick.count > 14 => return 0,
                _ => {},
            }
        }
        return g.id;
    }).sum();
    println!("part1: {p1}");
}

pub fn part2() {
    let p2 :i64 = get_games(DAY02_INPUT).iter().map(|g| {
        g.turns.iter().map(|t| &t.picks).flatten().group_by(|p| &p.color).into_iter().map(|g| {
            g.1.into_iter().map(|p| p.count).max().unwrap()
        }).product::<i64>()
    }).sum();

    println!("part2: {p2}");
}