use crate::helper::vec2::Vec2;
use lazy_static::lazy_static;
use regex::Regex;

const DAY13_INPUT: &str = include_str!(r"input\day13.txt");

lazy_static! {
    static ref DOUBLE_NEWLINE: Regex = Regex::new(r"(\r\n){2}|\n{2}").unwrap();
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<char>>,
    size: Vec2<usize>,
}

impl Map {
    fn new(input: &str) -> Self {
        let size = Vec2::new(
            input
                .lines()
                .next()
                .expect("could not find first line")
                .chars()
                .count(),
            input.lines().count(),
        );
        let tiles = input.lines().map(|l| l.chars().collect()).collect();
        Map { tiles, size }
    }

    fn get_symmetry(&self) -> Vec2<usize> {
        //columns
        for x in 0..self.size.x {
            if self.is_symmetric_over(x, false) {
                return Vec2::new(x, 0);
            }
        }

        for y in 0..self.size.y {
            if self.is_symmetric_over(y, true) {
                return Vec2::new(0, y);
            }
        }
        panic!("could not find symmetry for map: {:?}", self)
    }

    fn is_symmetric_over(&self, start: usize, is_row: bool) -> bool {
        if is_row {
            if start == 0 || start + 1 == self.size.y {
                return false;
            }

            for y in start + 1..self.size.y {
                let distance = y - start - 1;
                if start < distance {
                    break;
                }
                let opposite = start - distance;
                for x in 0..self.size.x {
                    if self.tiles[x][y] != self.tiles[x][opposite] {
                        return false;
                    }
                }
            }
            return true;
        } else {
            if start == 0 || start + 1 == self.size.x {
                return false;
            }

            for x in start + 1..self.size.x {
                let distance = x - start - 1;
                if start < distance {
                    break;
                }
                let opposite = start - distance;
                for y in 0..self.size.y {
                    if self.tiles[x][y] != self.tiles[opposite][y] {
                        return false;
                    }
                }
            }
            return true;
        }
    }
}

fn get_maps(input: &str) -> Vec<Map> {
    DOUBLE_NEWLINE.split(input).map(|i| Map::new(i)).collect()
}

pub fn part1() -> usize {
    let out: Vec2<_> = get_maps(DAY13_INPUT)
        .into_iter()
        .map(|m| m.get_symmetry())
        .sum();
    return out.x + 100 * out.y;
}

pub fn part2() -> i32 {
    0
}
