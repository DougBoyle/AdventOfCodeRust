use std::collections::{hash_map::Entry, BTreeMap, BTreeSet, HashMap, HashSet};

use aoc_2024::read_input;

fn main() {
    part1();
    part2();
}

fn part1() {
    let Input { available_towels, designs } = parse_input();

    let total = designs.iter().filter(|design| ways_to_make_design(design, &available_towels) > 0).count();

    println!("Total: {total}"); // 258
}

fn part2() {
    let Input { available_towels, designs } = parse_input();

    let total: usize = designs.iter().map(|design| ways_to_make_design(design, &available_towels)).sum();

    println!("Total: {total}"); // 632423618484345
}

fn ways_to_make_design(design: &Vec<char>, available_towels: &Vec<Vec<char>>) -> usize {
    let mut indices_to_search = BTreeMap::new();
    indices_to_search.insert(0, 1);
    loop {
        match indices_to_search.pop_first() {
            None => return 0,
            Some((current_index, count)) if current_index == design.len() => return count,
            Some((current_index, count)) => {
                for towel in available_towels.iter().filter(|towel| matches_design_segment(design, current_index, towel)) {
                    let end = current_index + towel.len();
                    indices_to_search.entry(end).and_modify(|counter| *counter += count).or_insert(count);
                }
            }
        }
    }
}

fn matches_design_segment(design: &Vec<char>, index: usize, towel: &Vec<char>) -> bool {
    let end = index + towel.len();
    design.len() >= end && design[index..end] == *towel
}

struct Input {
    available_towels: Vec<Vec<char>>,
    designs: Vec<Vec<char>>,
}

fn parse_input() -> Input {
    let mut lines = read_input(19);
    let available_towels = lines.next().unwrap().split(", ").map(|s| s.chars().collect()).collect();
    lines.next();
    let designs = lines.map(|s| s.chars().collect()).collect();
    Input { available_towels, designs }
}
