const _DAY10_SIMPLE_INPUT: &str = include_str!(r"..\input\day10_simple.txt");
const DAY10_INPUT: &str = include_str!(r"..\input\day10.txt");

mod pipemap {
    use std::collections::HashSet;

    use crate::vec2::Vec2;
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct Pipes: u32 {
            const NORTH = 0x01;
            const SOUTH = 0x02;
            const EAST = 0x04;
            const WEST = 0x08;
            const START = 0x10;
        }
    }

    #[derive(Debug)]
    pub struct PipeMap {
        map: Vec<Pipes>,
        width: i32,
        height: i32,
    }

    impl PipeMap {
        pub fn new(input: &str) -> Self {
            let width = input
                .lines()
                .next()
                .expect("could not find first line")
                .len()
                .try_into()
                .expect("width does not fit in i32");
            let mut height = 0;
            let map = input
                .lines()
                .flat_map(|l| {
                    height += 1;
                    l.chars()
                })
                .map(|c| match c {
                    '|' => Pipes::NORTH | Pipes::SOUTH,
                    '-' => Pipes::EAST | Pipes::WEST,
                    'L' => Pipes::NORTH | Pipes::EAST,
                    'J' => Pipes::NORTH | Pipes::WEST,
                    '7' => Pipes::SOUTH | Pipes::WEST,
                    'F' => Pipes::SOUTH | Pipes::EAST,
                    'S' => Pipes::START,
                    '.' => Pipes::empty(),
                    _ => panic!("unknown pipe type: {}", c),
                })
                .collect();
            let result = PipeMap { map, width, height };
            let _test: i32 = result
                .map
                .len()
                .try_into()
                .expect("map too big to fit in i32");
            return result;
        }

        pub fn get_direction_of_connected_pipes(&self, point: Vec2) -> HashSet<Vec2> {
            self.get_with_index(self.to_index(point))
        }

        pub fn get_dimensions(&self) -> Vec2 {
            return Vec2::new(self.width, self.height);
        }

        fn get_directions_for_start(&self, point: Vec2) -> HashSet<Vec2> {
            Vec2::ALL_CARDINAL
                .iter()
                .filter_map(|&d| {
                    self.get_direction_of_connected_pipes(point + d)
                        .contains(&d.invert())
                        .then_some(d)
                })
                .collect()
        }

        pub fn find_start(&self) -> Vec2 {
            self.from_index(
                self.map
                    .iter()
                    .position(|&p| p == Pipes::START)
                    .expect("could not find start")
                    .try_into()
                    .unwrap(),
            )
        }

        pub fn get_with_index(&self, index: i32) -> HashSet<Vec2> {
            if index < 0 || index >= self.map.len().try_into().unwrap() {
                HashSet::<_>::new()
            } else {
                let out = *self.map.get::<usize>(index.try_into().unwrap()).unwrap();
                if out.contains(Pipes::START) {
                    self.get_directions_for_start(self.from_index(index))
                } else {
                    Self::pipes_to_dir(out)
                }
            }
        }

        fn pipes_to_dir(pipes: Pipes) -> HashSet<Vec2> {
            pipes
                .into_iter()
                .map(|p| match p {
                    Pipes::NORTH => Vec2::NORTH,
                    Pipes::SOUTH => Vec2::SOUTH,
                    Pipes::EAST => Vec2::EAST,
                    Pipes::WEST => Vec2::WEST,
                    _ => panic!("failed to convert"),
                })
                .collect()
        }

        fn to_index(&self, point: Vec2) -> i32 {
            if point.x < 0 || point.x >= self.width || point.y < 0 || point.y >= self.height {
                -1
            } else {
                point.x + point.y * self.width
            }
        }

        fn from_index(&self, index: i32) -> Vec2 {
            Vec2 {
                x: index % self.width,
                y: index / self.width,
            }
        }
    }
}

use crate::vec2::Vec2;
use pipemap::*;
use std::collections::{HashMap, HashSet, VecDeque};

type Visited = HashMap<Vec2, i32>;

fn visit_loop(pipemap: &PipeMap, start: Vec2) -> (Visited, Vec2) {
    let mut visited = Visited::new();
    visited.insert(start, 0);
    let mut queue = VecDeque::<Vec2>::new();
    let mut last = start;
    queue.push_back(start);
    loop {
        let Some(current) = queue.pop_front() else {
            break;
        };
        let depth = *visited
            .get(&current)
            .expect("must already be visited to be in queue");
        for adj in pipemap.get_direction_of_connected_pipes(current) {
            let next = adj + current;
            if !visited.contains_key(&next) {
                visited.insert(next, depth + 1);
                last = adj + current;
                queue.push_back(next)
            }
        }
    }
    (visited, last)
}

pub fn part1() -> i32 {
    let pipemap = PipeMap::new(DAY10_INPUT);
    let start = pipemap.find_start();
    let (visited, last) = visit_loop(&pipemap, start);
    *visited.get(&last).unwrap()
}

//assume we're at top left corner of square, need to test two pipes to see if we can move between them
fn flood_fill(pipemap: &PipeMap, visited: &Visited) -> HashSet<Vec2> {
    let start = Vec2::new(0, 0);
    let mut flooded = HashSet::new();
    let mut queue = VecDeque::<Vec2>::new();
    flooded.insert(start);
    queue.push_back(start);
    let map_size = pipemap.get_dimensions() + Vec2::new(1, 1);
    loop {
        let Some(current) = queue.pop_front() else {
            break;
        };
        for dir in Vec2::ALL_CARDINAL {
            let next = current + *dir;

            if !is_in_map(&next, &map_size) || flooded.contains(&next) {
                continue;
            }

            let cannot_move =
                match *dir {
                    //up and left+right
                    Vec2::NORTH => get_pipes_if_visited(&(current + *dir), pipemap, visited)
                        .contains(&Vec2::WEST),
                    //same and left+right
                    Vec2::SOUTH => {
                        get_pipes_if_visited(&(current), pipemap, visited).contains(&Vec2::WEST)
                    }
                    //left and up + down
                    Vec2::WEST => get_pipes_if_visited(&(current + *dir), pipemap, visited)
                        .contains(&Vec2::NORTH),
                    //same and up + down
                    Vec2::EAST => {
                        get_pipes_if_visited(&(current), pipemap, visited).contains(&Vec2::NORTH)
                    }
                    _ => panic!(),
                };
            if cannot_move {
                continue;
            }
            flooded.insert(next);
            queue.push_back(next);
        }
    }

    return flooded;
}

fn get_pipes_if_visited(pos: &Vec2, pipemap: &PipeMap, visited: &Visited) -> HashSet<Vec2> {
    match visited.contains_key(pos) {
        true => pipemap.get_direction_of_connected_pipes(*pos),
        false => HashSet::new(),
    }
}

fn is_in_map(pos: &Vec2, map_size: &Vec2) -> bool {
    pos.x >= 0 && pos.x < map_size.x && pos.y >= 0 && pos.y < map_size.y
}

pub fn part2() -> usize {
    let pipemap = PipeMap::new(DAY10_INPUT);
    let start = pipemap.find_start();
    let (visited, _) = visit_loop(&pipemap, start);
    let flooded = flood_fill(&pipemap, &visited);
    let dimensions = pipemap.get_dimensions();
    let mut count = 0;
    for y in 0..dimensions.y {
        for x in 0..dimensions.x {
            let pos = Vec2::new(x, y);
            if !flooded.contains(&pos) && !visited.contains_key(&pos) {
                count += 1
            }
        }
    }
    count
}
