use std::collections::HashMap;

use regex::Regex;

const ONE: &str = "one";
const TWO: &str = "two";
const THREE: &str = "three";
const FOUR: &str = "four";
const FIVE: &str = "five";
const SIX: &str = "six";
const SEVEN: &str = "seven";
const EIGHT: &str = "eight";
const NINE: &str = "nine";

fn main() {
    let processor = LineProcessor2::new();

    let total: u32 = rust_aoc::read_input(1).map(|line| processor.process(line)).sum();

    // 1. 54953
    // 2. 53868
    println!("Total: {total}");
}

trait LineProcessor {
    fn process(&self, line: String) -> u32;
}

struct LineProcessor1;

impl LineProcessor for LineProcessor1 {
    fn process(&self, line: String) -> u32 {
        let digits = line.chars().filter_map(|c| c.to_digit(10));
        let (first, last) = LineProcessor1::first_and_last(digits);
        as_number(first, last)
    }
}

impl LineProcessor1 {
    fn first_and_last<T: Copy>(iter: impl std::iter::Iterator<Item=T>) -> (T, T) {
        let mut iter = iter.peekable();
        let first = *iter.peek().unwrap();
        let last = iter.last().unwrap();
        (first, last)
    }
}

struct LineProcessor2 {
    text_strings: HashMap<&'static str, u32>,
    re: Regex
}

impl LineProcessor for LineProcessor2 {
    fn process(&self, line: String) -> u32 {
        let first = self.re.find(&line).unwrap();
        let first_val = self.match_as_value(&first);

        let mut last_val = first_val;
        let mut last_start = first.start(); 
        while let Some(m) = self.re.find_at(&line, last_start + 1) {
            last_val = self.match_as_value(&m);
            last_start = m.start();
        }

        as_number(first_val, last_val)
    }
}

impl LineProcessor2 {
    fn new() -> LineProcessor2 {
        let mut text_strings = std::collections::HashMap::new();
        text_strings.insert(ONE, 1);
        text_strings.insert(TWO, 2);
        text_strings.insert(THREE, 3);
        text_strings.insert(FOUR, 4);
        text_strings.insert(FIVE, 5);
        text_strings.insert(SIX, 6);
        text_strings.insert(SEVEN, 7);
        text_strings.insert(EIGHT, 8);
        text_strings.insert(NINE, 9);

        let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();

        LineProcessor2 { text_strings, re }
    }

    fn match_as_value(&self, m: &regex::Match) -> u32 {
        let m = m.as_str();
        self.text_strings.get(m).map_or_else(|| LineProcessor2::string_as_digit(&m), |v| *v)
    }

    fn string_as_digit(str: &str) -> u32 {
        assert!(str.len() == 1);
        str.chars().next().unwrap().to_digit(10).unwrap()
    }
}

fn as_number(first: u32, second: u32) -> u32 {
    10*first + second
}
