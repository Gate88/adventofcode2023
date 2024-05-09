use std::ops::{Add, Sub};

const _DAY09_SIMPLE_INPUT: &str = include_str!(r"input\day09_simple.txt");
const DAY09_INPUT: &str = include_str!(r"input\day09.txt");

mod history {
    use std::marker::PhantomData;

    type HistoryItems = Vec<Vec<i32>>;

    pub struct New;
    pub struct Ran;
    pub struct History<State> {
        pub items: HistoryItems,
        state: PhantomData<State>,
    }

    impl History<New> {
        pub fn new(line: &str) -> Self {
            let mut items = Vec::new();

            let values = line
                .split(" ")
                .map(|v| v.parse().expect("could not parse number"))
                .collect();

            items.push(values);

            History {
                items,
                state: PhantomData,
            }
        }

        pub fn was_ran(self) -> History<Ran> {
            History {
                items: self.items,
                state: PhantomData,
            }
        }
    }
}

use history::*;

fn get_histories(input: &str) -> Vec<History<New>> {
    input.lines().map(|l| History::new(l)).collect()
}

fn iterate_history(mut history: History<New>) -> History<New> {
    let last = history
        .items
        .last()
        .expect("could not find line to iterate");

    let next = last
        .windows(2)
        .filter_map(|w| TryInto::<&[_; 2]>::try_into(w).ok())
        .map(|[a, b]| b - a)
        .collect();

    history.items.push(next);

    return history;
}

fn run_history(mut history: History<New>) -> History<Ran> {
    while !history.items.last().unwrap().iter().all(|&i| i == 0) {
        history = iterate_history(history);
    }

    history.was_ran()
}

fn extrapolate(history: &History<Ran>, forwards: bool) -> i32 {
    history
        .items
        .iter()
        .rev()
        .skip(1) // skip all 0s line
        .fold(
            0,
            if forwards {
                forwards_total
            } else {
                backwards_total
            },
        )
}

fn forwards_total<T>(total: T, iter: &Vec<T>) -> T
where
    T: Add<T, Output = T> + Copy,
{
    total + *iter.last().unwrap()
}

fn backwards_total<T>(total: T, iter: &Vec<T>) -> T
where
    T: Sub<Output = T> + Copy,
{
    *iter.first().unwrap() - total
}

pub fn part1() -> i32 {
    get_histories(DAY09_INPUT)
        .into_iter()
        .map(|h| extrapolate(&run_history(h), true))
        .sum()
}

pub fn part2() -> i32 {
    get_histories(DAY09_INPUT)
        .into_iter()
        .map(|h| extrapolate(&run_history(h), false))
        .sum()
}
