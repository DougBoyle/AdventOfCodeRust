use std::collections::{HashMap, HashSet};

use aoc_2024::read_input;
use rust_aoc::split_in_two;


fn main() {
    part1();
    part2();
}

fn part1() {
    let input = parse_input();
    let total: u32 = input.updates.iter()
        .filter(|update| is_correctly_ordered(update, &input.rules))
        .map(|update| update[update.len() / 2])
        .sum();

    println!("Total: {total}"); // 6034
}

fn part2() {
    let input = parse_input();
    let total: u32 = input.updates.iter()
        .filter(|update| !is_correctly_ordered(update, &input.rules))
        .map(|update| build_correct_order(update, &input.rules))
        .map(|update| update[update.len() / 2])
        .sum();

    println!("Total: {total}"); // 6305
}

fn is_correctly_ordered(update: &Vec<u32>, rules: &Rules) -> bool {
    update.iter().enumerate().all(|(i, value)| {
        if let Some(must_come_after) = rules.get(value) {
            update[..i].iter().all(|x| !must_come_after.contains(x))
        } else {
            true
        }
    })
}

/// Naively swap values until we get a solution.
fn build_correct_order(values: &Vec<u32>, rules: &Rules) -> Vec<u32> {
    let mut result = values.clone();
    while let Some((i, j)) = result.iter().enumerate().filter_map(|(i, value)| {
            let must_come_after = rules.get(value)?;
            result[..i].iter().enumerate().filter(|(_, x)| must_come_after.contains(x)).map(|(j, _)| (i, j)).next()
        }).next() {
        result.swap(i, j);
    }
    
    result
}

type Rules = HashMap<u32, HashSet<u32>>;

struct Input {
    rules: Rules,
    updates: Vec<Vec<u32>>,
}

fn parse_input() -> Input {
    let mut lines = read_input(5);

    let mut rules = HashMap::new();
    for s in lines.by_ref().take_while(|line| !line.trim().is_empty()) {
        let (n, m) = split_in_two(&s, '|');
        let (n, m) = (n.parse().unwrap(), m.parse().unwrap());
        rules.entry(n).or_insert(HashSet::new()).insert(m);
    }

    let updates = lines.map(|s| s.split(",").map(|n| n.parse().unwrap()).collect()).collect();

    Input { rules, updates }
}
