use std::collections::HashMap;

use aoc_2024::read_input;
use rust_aoc::MultiHashSet;

fn main() {
    part1();
    part2();
}

const ITERATIONS: usize = 2000;
const SEQ_LEN: usize = 4;

fn part1() {
    let initial_values = parse_input();
    let mut values = initial_values;
    for _ in 0..ITERATIONS {
        for value in &mut values {
            *value = next_value(*value);
        }
    }
    let total: u64 = values.into_iter().sum();
    println!("Total {total}"); // 17005483322
}

fn part2() {
    let initial_values = parse_input();
    let mut sequence_total_prices = MultiHashSet::new();
    for (sequence, price) in initial_values.into_iter().flat_map(|value| prices_for_each_sequence(value).into_iter()) {
        sequence_total_prices.insert_many(sequence, price as usize);
    }
    let max_total_price = sequence_total_prices.into_iter().map(|(_, total_price)| total_price).max().unwrap();
    println!("Total {max_total_price}"); // 1910
}

fn prices_for_each_sequence(initial_value: u64) -> HashMap<[i32; SEQ_LEN], i32> {
    let initial_price = (initial_value % 10) as i32;
    let mut prices = Vec::new();
    let mut current = initial_value;
    for _ in 0..ITERATIONS {
        current = next_value(current);
        prices.push((current % 10) as i32);
    }
    let changes: Vec<i32> = prices.iter().enumerate()
        .map(|(i, price)| if i == 0 { price - initial_price } else { price - prices[i-1] })
        .collect();
    let mut prices_for_sequences = HashMap::new();
    for i in 0..ITERATIONS-SEQ_LEN {
        let sequence = changes[i..i+4].try_into().unwrap();
        let price = prices[i + 3];
        if !prices_for_sequences.contains_key(&sequence) {
            prices_for_sequences.insert(sequence, price);
        }
    }
    prices_for_sequences
}

const DIVISOR: u64 = 16777216;

fn next_value(value: u64) -> u64 {
    let value = value ^ (value * 64) % DIVISOR;
    let value = value ^ (value / 32) % DIVISOR;
    value ^ (value * 2048) % DIVISOR
}

fn parse_input() -> Vec<u64> {
    read_input(22).map(|line| line.parse().unwrap()).collect()
}
