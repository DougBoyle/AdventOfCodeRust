use std::{collections::HashMap, io::{Error, ErrorKind}, str::FromStr};

fn main() {
    println!("Part 1");
    Part1::process();
    println!("Part 2");
    Part2::process();
}

trait Part {
    fn parse(s: String) -> Configuration;

    fn process() {
        let total: usize = rust_aoc::read_input(12).map(Self::parse)
            .map(Configuration::permutations)
            .sum();
        // 1. 6827
        // 2. 1537505634471
        println!("Total ways: {total}");
    }
}

struct Part1;

impl Part for Part1 {
    fn parse(s: String) -> Configuration {
        s.parse().unwrap()
    }
}

struct Part2;

impl Part for Part2 {
    fn parse(s: String) -> Configuration {
        let config: Configuration = s.parse().unwrap();
        let mut states = config.states.clone();
        for _ in 1..5 {
            states.push(State::Unknown);
            states.append(&mut config.states.clone());
        }
        let mut groups = config.groups.clone();
        for _ in 1..5 {
            groups.append(&mut config.groups.clone());
        }
        Configuration { states, groups }
    }
}

struct MemoTable {
    configuration: Configuration,
    // (state, group) -> permutations for {states[state..], groups[group..]}
    permutations: HashMap<(usize, usize), usize>
}

impl MemoTable {
    fn new(configuration: Configuration) -> MemoTable {
        MemoTable { configuration, permutations: HashMap::new() }
    }

    fn get(&mut self, state: usize, group: usize) -> usize {
        match self.permutations.get(&(state, group)) {
            Some(perms) => *perms,
            None => {
                let perms = self.calc_permutations(state, group);
                self.permutations.insert((state, group), perms);
                perms
            }
        }
    }

    fn calc_permutations(&mut self, state: usize, group: usize) -> usize {
        let states = &self.configuration.states;

        match states.get(state) {
            None => if group == self.configuration.groups.len() { 1 } else { 0 }
            Some(State::Blank) => self.get(state + 1, group),
            Some(State::Occupied) => self.calc_permutations_occupied(state, group),
            Some(State::Unknown) => {
                let occupied_perms = self.calc_permutations_occupied(state, group);
                let blank_perms = self.get(state + 1, group);
                occupied_perms + blank_perms
            },
        }
    }

    /// Assumes configurations.states[state] is Occupied, without checking, as it is used to handle both Occupied and Unknown.
    fn calc_permutations_occupied(&mut self, mut state: usize, group: usize) -> usize {
        let states = &self.configuration.states;

        let run = self.configuration.groups.get(group);
        match run {
            None => 0,
            Some(run) => {
                let run = *run;
                for i in 1..run {
                    match states.get(state + i) {
                        None | Some(State::Blank) => { return 0 },
                        _ => {}
                    }
                }
                state += run;
                match states.get(state) {
                    Some(State::Occupied) => { return 0 },
                    Some(_) => { state += 1 },
                    None => {},
                }
                self.get(state, group + 1)
            },
        }
    }
}

#[derive(Clone, Debug)]
struct Configuration {
    states: Vec<State>,
    groups: Vec<usize>,
}

impl Configuration {
    fn permutations(self) -> usize {
        MemoTable::new(self).get(0, 0)
    }
}

impl FromStr for Configuration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (states, groups) = rust_aoc::split_in_two(s, ' ');
        let states = states.chars().map(State::try_from).map(Result::unwrap).collect();
        let groups = groups.split(',').map(str::parse).map(Result::unwrap).collect();
        Ok(Configuration { states, groups })
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum State {
    Blank,
    Occupied,
    Unknown,
}

impl TryFrom<char> for State {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(State::Blank),
            '#' => Ok(State::Occupied),
            '?' => Ok(State::Unknown),
            _ => Err(Error::new(ErrorKind::InvalidInput, format!("Unknown state {value}")))
        }
    }
}
