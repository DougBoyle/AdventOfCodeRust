use std::{collections::{BTreeSet, HashSet, VecDeque}, io::{Error, ErrorKind}, str::FromStr};

fn main() {
    println!("PART 1");
    Part1::process();
    println!("PART 2");
    Part2::process();
}

trait Part {
    fn parse(s: String) -> Configuration;

    fn process() {
        let total: usize = rust_aoc::read_input(12).map(Self::parse)
            .map(|conf| { println!("Solving {conf:?}"); conf })
            .map(Configuration::permutations)
            .map(|n| { println!("{n}"); n })
            .sum();
        // 1. 6827
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

// TODO: Would it help to memo-ize some parts of it?
impl Part for Part2 {
    fn parse(s: String) -> Configuration {
        let config: Configuration = s.parse().unwrap();
        let mut states = config.states.clone();
        for _ in 1..5 {
            states.push_back(State::Unknown);
            states.append(&mut config.states.clone());
        }
        let mut groups = config.groups.clone();
        for _ in 1..5 {
            groups.append(&mut config.groups.clone());
        }
        Configuration { states, groups }
    }
}

#[derive(Clone, Debug)]
struct Configuration {
    states: VecDeque<State>,
    groups: VecDeque<usize>,
}

// TODO: Rather than every branch, just pick next tile to start group on?
//       i.e. if we split at a ?, can simplify next cases as: ? is #, or ? is blank and drop all blanks immediately after too
impl Configuration {
    /// '#' at ether end of States is always the start/end of the group.
    /// Check validity before calling?
    fn permutations(mut self) -> usize {
        // Drop blanks
        while !self.states.is_empty() && self.states[0] == State::Blank { self.states.pop_front(); }

        // Base case: empty states
        if self.states.is_empty() {
            return if self.groups.is_empty() { 1 } else { 0 }
        }

        // avoid recursing down the wrong branch too far
        // TODO: Improve the minimum, step forwards on known blank tiles when predicting filling in runs?
        if self.states.len() < Configuration::minimum_length(&self.groups) { return 0 }

        // simplify
        match &self.states[0] {
            State::Blank => panic!("Bug: Should have discarded blanks already"),
            State::Occupied => {
                // discard first group if possible, else refute
                let run = self.groups.pop_front();
                match run {
                    None => 0,
                    Some(run) => {
                        // TODO: 'pop_run' function?
                        for _ in 0..run {
                            match self.states.pop_front() {
                                None | Some(State::Blank) => { return 0 },
                                _ => {}
                            }
                        }
                        // and one more for the separator after the run, here None is allowed if we hit the end
                        match self.states.pop_front() {
                            Some(State::Occupied) => { return 0 },
                            _ => {}
                        }
                        self.permutations()
                    },
                }
            },
            State::Unknown => {
                // Treat as occupied
                let mut occupied = self.clone();
                occupied.states[0] = State::Occupied;
                // Treat as blank
                let mut blank = self;
                blank.states.pop_front();
                blank.permutations() + occupied.permutations()
            },
        }
    }

    // TODO: Be smarter about this?
    fn minimum_length(groups: &VecDeque<usize>) -> usize {
        if groups.len() == 0 { return 0 }
        groups.iter().sum::<usize>() + groups.len() - 1
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
