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
#[derive(Clone, Debug)]
struct Range {
    start: usize,
    len: usize,
}

impl Range {
    fn new(start: usize, len: usize) -> Self {
        Range {
            start, len
        }
    }

    fn end(&self) -> usize {
        self.start + self.len
    }

    fn in_range(&self, value: usize) -> bool {
        value >= self.start && value < self.end()
    }
}

struct Entry {
    destination: Range,
    source: Range,
}

impl Entry {
    fn apply_range(&self, value: usize) -> usize {
        if self.source.in_range(value) {
            value - self.source.start + self.destination.start
        } else {
            value
        }
    }

    //returns the converted ranges, and then the leftover range
    fn get_ranges(&self, range: &Range) -> (Option<Vec<Range>>,Option<Range>) {
        if self.source.in_range(range.start) && self.source.in_range(range.end()-1) {
            return (Some(vec![Range {
                start: self.apply_range(range.start),
                len: range.len
            }]), None)
        } else if self.source.in_range(range.start) {
            let new_len = self.source.end() - range.start;
            return (Some(vec![
                Range {
                    start: self.apply_range(range.start),
                    len: new_len,  
                }
            ]), Some(Range {
                start: range.start + new_len,
                len: range.len - new_len,
            }))
        } else if self.source.in_range(range.end()-1) {
            return (Some(vec![Range {
                start: range.start,
                len: self.source.end() - range.start,
            }, Range {
                start: self.apply_range(self.source.start),
                len: self.source.len,
            }
            ]), None)
        } else {
            return (Some(vec![
                range.clone()
            ]), None)
        }
    }
}

//key for BTreeMap is source, bc we want to look up by source. SORTED.
type EntryList = Vec<Entry>;
//key for HashMap is source name, bc we want to look up by source
type Lookup<'a> = HashMap<&'a str, EntryList>;

struct Almanac<'a> {
    start_category: &'a str,
    start_values: Vec<usize>,
    start_ranges: Vec<Range>,
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
        let start_ranges: Vec<Range> = start_values.chunks_exact(2).map(|c| Range { start: c[0], len: c[1]}).collect();

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
                    destination: Range::new(destination, range_length),
                    source: Range ::new(source, range_length)
                });
            }
            lookup.insert(source_name, entry_list.into_iter().map(|i| i.1).collect());
        }

        Almanac {
            start_category: "seed",
            start_values,
            start_ranges,
            source_to_destination_name,
            lookup,
        }
    }

    fn apply_category_ranges(&self, category: &str, ranges: &Vec<Range>) -> Option<(Vec<Range>, &str)> {
        let Some(destination_name) = self.get_next_category(category) else { return None };
        let mut out = Vec::new();

        for range in ranges {
            out.append(&mut self.apply_category_range(category, range));
        }

        Some((out, destination_name))
    }

    fn apply_category_range(&self, category: &str, range: &Range) -> Vec<Range> {
        let entry_list = self.lookup.get(category).unwrap();
        let mut out = Vec::new();

        let mut i = entry_list.partition_point(|e| range.start >= e.source.end());
        if i == entry_list.len() {
            out.push(range.clone());
        } else {
            let mut c = range.clone();
            loop {
                if i >= entry_list.len() {
                    out.push(c);
                    break;
                }
                let entry = &entry_list[i];
                let ranges = entry.get_ranges(&c);
                if let Some(mut list) = ranges.0 {
                    out.append(&mut list);
                }
                if let Some(continuing_range) = ranges.1 {
                    c = continuing_range;
                    i += 1;
                } else {
                    break;
                }
            }    
        }

        out
    }

    fn get_next_category(&self, category: &str) -> Option<&str> {
        self.source_to_destination_name.get(category).copied()
    }

    fn convert_forwards(&self, value: usize, category: &str) -> Option<(usize, &str)> {
        let Some(destination_name) = self.get_next_category(category) else { return None };
        let entry_list = self.lookup.get(category).unwrap();

        let i = entry_list.partition_point(|e| value >= e.source.end());
        if i != entry_list.len() {
            Some((entry_list[i].apply_range(value), destination_name))
        } else {
            Some((value, destination_name))
        }
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
    let alm = Almanac::new(DAY05_INPUT);
    let mut category = alm.start_category;
    let mut ranges = alm.start_ranges.clone();
    print_ranges(&ranges, category);
    while let Some(v) = alm.apply_category_ranges(category, &ranges) {
        ranges = v.0;
        category = v.1;
        print_ranges(&ranges, category);
    }
    let min = ranges.iter().min_by_key(|r| r.start).unwrap();
    println!("part2: {} = {}", category, min.start);
}

fn print_ranges(ranges: &Vec<Range>, category: &str) {
    println!("{category}: ");
    println!("{:?}", ranges);
    println!();
}