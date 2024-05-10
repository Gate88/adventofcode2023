use crate::days::day11::universe::Universe;

use self::universe::ManhattanDistance;

const DAY11_INPUT: &str = include_str!(r"input\day11.txt");
const _DAY11_INPUT_SIMPLE: &str = include_str!(r"input\day11_simple.txt");

mod universe {
    type Vec2 = crate::helper::vec2::Vec2<i64>;
    use std::collections::HashSet;

    #[derive(Debug)]
    pub struct Universe {
        map: HashSet<Vec2>,
    }

    impl Universe {
        pub fn new(input: &str) -> Self {
            let map = input
                .lines()
                .enumerate()
                .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)))
                .filter_map(|(x, y, c)| (c == '#').then_some(Vec2::new(x as i64, y as i64)))
                .collect();

            Universe { map }
        }

        pub fn get_galaxies(&self) -> impl Iterator<Item = &Vec2> {
            self.map.iter()
        }

        pub fn expand(&mut self, expanded_size: i64) {
            let (by_x, by_y): (HashSet<_>, HashSet<_>) =
                self.map.iter().map(|v| (v.x, v.y)).unzip();

            let mut rows: Vec<_> = by_y.iter().map(|i| *i).collect();
            rows.sort_unstable();
            let rows = rows;

            let mut columns: Vec<_> = by_x.iter().map(|i| *i).collect();
            columns.sort_unstable();
            let columns = columns;

            let mut expanded_map = HashSet::new();
            for galaxy in &self.map {
                let galaxies_left_and_above = Vec2::new(
                    columns.partition_point(|&x| x < galaxy.x) as i64,
                    rows.partition_point(|&y| y < galaxy.y) as i64,
                );

                let empty_left_and_above = *galaxy - galaxies_left_and_above;

                expanded_map.insert(*galaxy + empty_left_and_above * (expanded_size - 1));
            }

            self.map = expanded_map;
        }
    }

    pub trait ManhattanDistance {
        fn manhattan_distance(&self, rhs: &Self) -> i64;
    }

    impl ManhattanDistance for Vec2 {
        fn manhattan_distance(&self, rhs: &Self) -> i64 {
            (rhs.x - self.x).abs() + (rhs.y - self.y).abs()
        }
    }
}

pub fn part1() -> i64 {
    let mut universe = Universe::new(DAY11_INPUT);
    universe.expand(2);
    calc_pair_distances(universe)
}

fn calc_pair_distances(universe: Universe) -> i64 {
    let mut total = 0;
    for (s, g1) in universe.get_galaxies().enumerate() {
        for g2 in universe.get_galaxies().skip(s + 1) {
            total += g1.manhattan_distance(g2);
        }
    }
    total
}

pub fn part2() -> i64 {
    let mut universe = Universe::new(DAY11_INPUT);
    universe.expand(1_000_000);
    calc_pair_distances(universe)
}
