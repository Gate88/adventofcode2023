use itertools::Itertools;

const DAY12_INPUT: &str = include_str!(r"input\day12.txt");
const _DAY12_INPUT_SIMPLE: &str = include_str!(r"input\day12_simple.txt");

#[derive(Debug)]
struct SpriteRecord {
    record: String,
    groups: Vec<usize>,
}

impl<'a> SpriteRecord {
    pub fn new(line: &str) -> Self {
        let mut split = line.split(" ");
        let record = split
            .next()
            .expect("could not find first record")
            .to_owned();
        let groups = split
            .next()
            .expect("could not find groups")
            .split(",")
            .map(|s| s.parse().expect("failed to parse"))
            .collect();

        SpriteRecord { record, groups }
    }

    pub fn new_folded_record(line: &str) -> Self {
        let mut sr = Self::new(line);
        let mut new_record = "".to_owned();
        let mut new_groups = Vec::new();
        for _ in 0..5 {
            new_record.push_str(&sr.record);

            new_groups.extend(sr.groups.iter());
        }
        sr.record = new_record;
        sr.groups = new_groups;

        return sr;
    }

    pub fn get_question_mark_indexes(&self) -> Vec<usize> {
        let question_mark_indexes = self
            .record
            .char_indices()
            .filter_map(|(i, c)| (c == '?').then_some(i))
            .collect();

        question_mark_indexes
    }

    pub fn get_possible_records(&self) -> Vec<String> {
        let picks =
            self.groups.iter().sum::<usize>() - self.record.chars().filter(|&c| c == '#').count();

        let question_marks = self.get_question_mark_indexes();
        question_marks
            .iter()
            .combinations(picks)
            .map(|p| {
                self.record
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if c == '?' {
                            if p.contains(&&i) {
                                '#'
                            } else {
                                if question_marks.contains(&i) {
                                    '.'
                                } else {
                                    '#'
                                }
                            }
                        } else {
                            c
                        }
                    })
                    .collect()
            })
            .collect_vec()
    }

    pub fn record_match_groups(&self, record: &str) -> bool {
        let mut group_started = None;
        let mut group_index = 0;
        let len = record.chars().count();

        for (i, c) in record.chars().enumerate().chain([(len, '!')]) {
            match (c, group_started) {
                ('#', None) => {
                    if group_index >= self.groups.len() {
                        return false;
                    }
                    group_started = Some(i)
                }
                ('.' | '!', Some(group_start)) => {
                    if i - group_start != self.groups[group_index] {
                        return false;
                    }
                    group_index += 1;
                    group_started = None;
                }
                ('#' | '.' | '!', _) => (),
                _ => panic!("unknown character: {}", c),
            };
        }

        group_index == self.groups.len()
    }
}

fn get_records(input: &str) -> Vec<SpriteRecord> {
    input.lines().map(|l| SpriteRecord::new(l)).collect()
}

fn get_folded_records(input: &str) -> Vec<SpriteRecord> {
    input
        .lines()
        .map(|l| SpriteRecord::new_folded_record(l))
        .collect()
}

pub fn part1() -> usize {
    let records = get_records(DAY12_INPUT);
    records
        .iter()
        .map(|r| {
            r.get_possible_records()
                .iter()
                .filter(|p| r.record_match_groups(p))
                .count()
        })
        .sum()
}

pub fn part2() -> usize {
    0
    // records
    //     .iter()
    //     .map(|r| {
    //         r.get_possible_records()
    //             .iter()
    //             .filter(|p| r.record_match_groups(p))
    //             .count()
    //     })
    //     .sum()
}
