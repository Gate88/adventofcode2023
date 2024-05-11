use memoize::memoize;

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
        let repeat = 5;
        for i in 0..repeat {
            new_record.push_str(&sr.record);
            if i < repeat - 1 {
                new_record.push_str(&"?");
            }
            new_groups.extend(sr.groups.iter());
        }
        sr.record = new_record;
        sr.groups = new_groups;

        return sr;
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

#[memoize]
fn count_ways(line: String, groups: Vec<usize>) -> usize {
    let count = line.chars().count();
    if count == 0 {
        //if string and groups are both empty, we're good
        if groups.len() == 0 {
            return 1;
        }
        //otherwise if we still have groups, this didn't work
        return 0;
    }

    //if groups are empty
    if groups.len() == 0 {
        //if there are any remaining '#', no good
        if line.chars().any(|c| c == '#') {
            return 0;
        }
        //if only '?' and '.' are left, then we're good
        return 1;
    }

    if count < groups.iter().map(|r| *r).sum::<usize>() + groups.len() - 1 {
        return 0;
    }

    match line.chars().next() {
        Some('.') => return count_ways(line.chars().skip(1).collect(), groups),
        Some('#') => match groups.as_slice() {
            [run, rest @ ..] => {
                let mut iter = line.chars();
                for c in iter.by_ref().take(*run) {
                    if c == '.' {
                        return 0;
                    }
                }
                //not possible for next character to be #
                if let Some(next) = iter.by_ref().next() {
                    if next == '#' {
                        return 0;
                    }
                }
                //if next character is '?' or '.' skip it; already account for '#' above
                return count_ways(line.chars().skip(*run + 1).collect(), rest.to_owned());
            }
            [] => panic!("shouldn't happen"),
        },
        Some('?') => {
            return count_ways(
                ['#'].into_iter().chain(line.chars().skip(1)).collect(),
                groups.clone(),
            ) + count_ways(
                ['.'].into_iter().chain(line.chars().skip(1)).collect(),
                groups,
            )
        }
        _ => panic!("shouldn't happne"),
    }
}

pub fn part1() -> usize {
    let records = get_records(DAY12_INPUT);
    records
        .iter()
        .map(|r| count_ways(r.record.to_owned(), r.groups.to_owned()))
        .sum()
}

pub fn part2() -> usize {
    let records = get_folded_records(DAY12_INPUT);
    records
        .iter()
        .map(|r| count_ways(r.record.to_owned(), r.groups.to_owned()))
        .sum()
}
