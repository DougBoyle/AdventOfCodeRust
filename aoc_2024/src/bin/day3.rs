use std::num::ParseIntError;

use regex::{Captures, Regex};

fn main() {
    part1();
    part2();
}

const PATTERN: &str = "do\\(\\)|don't\\(\\)|mul\\((\\d+),(\\d+)\\)";

fn part1() {
    let total: u32 = parse_input().map(|op| match op {
        Operation::Mult(a, b) => a*b,
         _ => 0,
    }).sum();

    println!("Total: {total}"); // 187833789
}

fn part2() {
    let mut enabled = true;
    let total: u32 = parse_input().map(|op| match op {
        Operation::Mult(a, b) => if enabled { a*b } else { 0 },
        Operation::Do => { enabled = true; 0 },
        Operation::Dont => { enabled = false; 0 }
    })
    .sum();

    println!("Total: {total}"); // 94455185
}

fn parse_input() -> impl Iterator<Item=Operation> {
    let re = Regex::new(PATTERN).unwrap();
    aoc_2024::read_input(3)
        .map(move |s| re.captures_iter(&s)
            .map(|cap| Operation::try_from(cap).unwrap()).collect::<Vec<_>>()
        )
        .flatten()
}

enum Operation {
    Do,
    Dont,
    Mult(u32, u32)
}

impl TryFrom<Captures<'_>> for Operation {
    type Error = ParseIntError;

    fn try_from(cap: Captures<'_>) -> Result<Self, Self::Error> {
        match &cap[0] {
            "do()" => Ok(Operation::Do),
            "don't()" => Ok(Operation::Dont),
            _ => Ok(Operation::Mult(cap[1].parse()?, cap[2].parse()?))
        }
    }
}

