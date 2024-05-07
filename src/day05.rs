use std::collections::{BTreeMap, HashMap};
use lazy_static::lazy_static;
use regex::Regex;

const _DAY05_SIMPLE_INPUT: &str = include_str!(r"..\input\day05_simple.txt");
const DAY05_INPUT: &str = include_str!(r"..\input\day05.txt");

lazy_static! {
    static ref SECTION_RE: Regex = Regex::new(r"(\r\n){2}|\r{2}|\n{2}").unwrap();
    static ref NAME_RE: Regex = Regex::new(r"(?P<source>.*)-to-(?P<destination>.*) map:").unwrap();
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
    static ref ENTRY_RE: Regex = Regex::new(r"(\d+)\s(\d+)\s(\d+)").unwrap();
}

struct Entry {
    destination: usize,
    source: usize,
    range_length: usize,
}

//key for BTreeMap is source, bc we want to look up by source. SORTED.
type EntryList = Vec<Entry>;
//key for HashMap is source name, bc we want to look up by source
type Lookup<'a> = HashMap<&'a str, EntryList>;

struct Almanac<'a> {
    start_category: &'a str,
    start_values: Vec<usize>,
    lookup: Lookup<'a>,
    source_to_destination_name: HashMap<&'a str, &'a str>,
}

impl<'a> Almanac<'a> {
    fn new(input: &'a str) -> Self {
        let mut lookup: Lookup = Lookup::new();
        let mut source_to_destination_name: HashMap<&'a str, &'a str> = HashMap::new();

        let mut sections = SECTION_RE.split(input);
        let seed_section = sections.next().unwrap();
        let start_values: Vec<usize> = NUM_RE.find_iter(seed_section).map(|s| s.as_str().parse::<usize>().unwrap()).collect();

        for section in sections {
            let captures = NAME_RE.captures(section).unwrap();
            let source_name = captures.name("source").unwrap().as_str();
            let destination_name = captures.name("destination").unwrap().as_str();
            source_to_destination_name.insert(source_name, destination_name);
            let mut entry_list = BTreeMap::new();
            for entry_match in ENTRY_RE.captures_iter(section) {
                let destination: usize = entry_match.get(1).unwrap().as_str().parse().unwrap();
                let source: usize = entry_match.get(2).unwrap().as_str().parse().unwrap();
                let range_length: usize = entry_match.get(3).unwrap().as_str().parse().unwrap();
                entry_list.insert(source, Entry {
                    destination,
                    source,
                    range_length,
                });
            }
            lookup.insert(source_name, entry_list.into_iter().map(|i| i.1).collect());
        }

        Almanac {
            start_category: "seed",
            start_values,
            source_to_destination_name,
            lookup,
        }
    }

    fn get_next_category(&self, category: &str) -> Option<&str> {
        self.source_to_destination_name.get(category).copied()
    }

    fn convert_forwards(&self, value: usize, category: &str) -> Option<(usize, &str)> {
        let Some(destination_name) = self.get_next_category(category) else { return None };
        let entry_list = self.lookup.get(category).unwrap();

        let i = entry_list.partition_point(|e| value >= e.source + e.range_length);
        if i != entry_list.len() {
            let entry = &entry_list[i];
            if value >= entry.source {
                return Some((value - entry.source + entry.destination, destination_name))
            }
        }
        Some((value, destination_name))
    }

    fn convert_all_forwards(&'a self, value: usize, category: &'a str) -> (usize, &'a str) {
        return self.convert_all(value, category, &Self::convert_forwards)
    }

    fn convert_all<F>(&'a self, value: usize, category: &'a str,f: F) -> (usize, &'a str)
    where F: Fn(&'a Self, usize, &str) -> Option<(usize, &'a str)>
    {
        let mut out = (value, category);
        if DEBUG { println!("{}: {}", out.1, out.0)}
        while let Some(next) = f(self, out.0, out.1) {
            out = next;
            if DEBUG { println!("{}: {}", out.1, out.0)}
        }
        if DEBUG { println!() }
        return out
    }

}

const DEBUG: bool = false;

pub fn part1() {
    let alm = Almanac::new(DAY05_INPUT);
    let p1 = alm.start_values.iter().map(|v| {
        alm.convert_all_forwards(*v, alm.start_category)
    }).min().unwrap();
    println!("part1: {} = {}", p1.1, p1.0)
}

pub fn part2() {

}