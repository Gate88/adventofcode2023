const _DAY06_SIMPLE_INPUT: &str = include_str!(r"..\input\day06_simple.txt");
const DAY06_INPUT: &str = include_str!(r"..\input\day06.txt");

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn shortest_hold_time(&self) -> usize {
        for hold in 1..self.time {
            if (self.time - hold) * hold > self.distance {
                return hold;
            }
        }
        return self.time;
    }

    fn longest_hold_time(&self) -> usize {
        for hold in (1..self.time).rev() {
            if (self.time - hold) * hold > self.distance {
                return hold;
            }
        }
        return self.time;
    }

    fn ways_to_win(&self) -> usize {
        1 + self.longest_hold_time() - self.shortest_hold_time()
    }
}

fn get_races(input: &str) -> Vec<Race> {
    let times = input
        .lines()
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter_map(|s| s.parse::<usize>().ok());
    let distances = input
        .lines()
        .nth(1)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter_map(|s| s.parse::<usize>().ok());

    times
        .zip(distances)
        .map(|i| Race {
            time: i.0,
            distance: i.1,
        })
        .collect()
}

fn get_long_race(input: &str) -> Race {
    let time = input
        .lines()
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<usize>()
        .unwrap();

    let distance = input
        .lines()
        .nth(1)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<usize>()
        .unwrap();

    Race { time, distance }
}

pub fn part1() -> String {
    let races = get_races(DAY06_INPUT);
    let p1: usize = races.iter().map(|r| r.ways_to_win()).product();
    format!("{}", p1)
}

pub fn part2() -> String {
    let race = get_long_race(DAY06_INPUT);
    format!("{}", race.ways_to_win())
}
