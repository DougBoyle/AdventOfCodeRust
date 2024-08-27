use std::{cmp::min, collections::{BTreeMap, HashSet}};
use std::ops::Bound::{Excluded, Unbounded};

fn main() {
    Part1::process();
    Part2::process();
}

struct Part1 {}

impl Part1 {
    fn process() {
        let mut lines = rust_aoc::read_input(5);

        let mut seeds = Part1::parse_seeds(&lines.next().unwrap());
        lines.next().unwrap(); // blank line

        while let Some(_heading) = lines.next() { // each map
            let mapping = parse_mapping(&mut lines);
            // apply to existing set
            seeds = seeds.iter().map(|v| mapping.get(*v)).collect();
        }

        let min_location = seeds.iter().min().unwrap();

        println!("Min Location: {min_location}"); // 175622908
    }

    fn parse_seeds(line: &str) -> HashSet<usize> {
        let (_, seeds) = rust_aoc::split_in_two(line, ':');
        seeds.split_ascii_whitespace().map(str::parse).map(Result::unwrap).collect()
    }
}

struct Part2 {}

impl Part2 {
    fn process() {
        let mut lines = rust_aoc::read_input(5);

        let mut seeds = Part2::parse_seeds(&lines.next().unwrap());
        lines.next().unwrap(); // blank line

        while let Some(_heading) = lines.next() { // each map
            let mapping = parse_mapping(&mut lines);

            seeds = seeds.iter().flat_map(|interval| Part2::map(*interval, &mapping).into_iter()).collect();
            seeds.sort_by_key(|interval| interval.start);
            seeds = Part2::coalesce(seeds);
        }

        let min_location = seeds.iter().next().unwrap().start;

        println!("Min Location: {min_location}"); // 5200543
    }

    fn parse_seeds(line: &str) -> Vec<Interval> {
        let (_, seeds) = rust_aoc::split_in_two(line, ':');
        let values: Vec<usize> = seeds.split_ascii_whitespace().map(str::parse).map(Result::unwrap).collect();
        let intervals = values.len() / 2;
        (0..intervals).map(|i| Interval { start: values[2*i], len: values[2*i + 1] }).collect()
    }

    fn map(mut interval: Interval, mapping: &Mapping) -> Vec<Interval> {
        let mut result = Vec::new();

        while interval.len > 0 {
            let mapped_interval = match mapping.get_line(interval.start) {
                None => {
                    match mapping.get_next_line_start(interval.start) {
                        None => interval,
                        Some(end) => {
                            let len = min(interval.len, end - interval.start);
                            Interval {start: interval.start, len}
                        }
                    }
                },
                Some(Line {dest_start, source_start, len}) => {
                    let line_end = source_start + len;
                    let overlap_end = min(interval.start + interval.len, line_end);
                    let overlap_len = overlap_end - interval.start;
                    Interval { start: interval.start - source_start + dest_start, len: overlap_len }
                }
            };

            result.push(mapped_interval);
            interval.start += mapped_interval.len;
            interval.len -= mapped_interval.len;
        }

        result
    }

    fn coalesce(intervals: Vec<Interval>) -> Vec<Interval> {
        let mut result = Vec::new();
        let mut it = intervals.into_iter();
        result.push(it.next().unwrap());
        for interval in it {
            let last = result.last_mut().unwrap();
            if interval.start == last.start + last.len {
                last.len += interval.len
            } else {
                result.push(interval)
            }
        }
        result
    }
}

fn parse_mapping<I: Iterator<Item=String>>(lines: &mut I) -> Mapping {
    lines.by_ref().take_while(|line| line.len() > 0)
        .map(|line| line.parse().unwrap())
        .map(|line: Line| (line.source_start, line))
        .collect()
}

struct Mapping {
    intervals_by_source: BTreeMap<usize, Line>
}

impl FromIterator<(usize, Line)> for Mapping {
    fn from_iter<T: IntoIterator<Item = (usize, Line)>>(iter: T) -> Self {
        Mapping { intervals_by_source: BTreeMap::from_iter(iter) }
    }
}

impl Mapping {
    fn get(&self, value: usize) -> usize {
        match self.intervals_by_source.range(0..=value).next_back() {
            None => value,
            Some((_, Line{ dest_start, source_start, len })) => {
                if value < source_start + len {
                    value + dest_start - source_start
                } else {
                    value
                }
            }
        }
    }

    fn get_line(&self, value: usize) -> Option<&Line> {
        match self.intervals_by_source.range(0..=value).next_back() {
            None => None,
            Some((_, line @ Line{ source_start, len, .. })) => {
                if value < source_start + len {
                    Some(line)
                } else {
                    None
                }
            }
        }
    }

    fn get_next_line_start(&self, value: usize) -> Option<usize> {
        match self.intervals_by_source.range((Excluded(value), Unbounded)).next() {
            None => None,
            Some((_, Line { source_start, .. })) => Some(*source_start)
        }
    }
}

struct Line {
    dest_start: usize,
    source_start: usize,
    len: usize,
}

impl std::str::FromStr for Line {
    type Err = ();
    fn from_str(s: &str) -> Result<Line, ()> {
        let values: Vec<usize> = s.split(' ').map(str::parse).map(Result::unwrap).collect();
        assert!(values.len() == 3);
        Ok(Line {
            dest_start: values[0],
            source_start: values[1],
            len: values[2]
        })
    }
}

#[derive(Copy, Clone)]
struct Interval { start: usize, len: usize }