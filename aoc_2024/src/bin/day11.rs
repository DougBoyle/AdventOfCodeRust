use std::collections::HashMap;

use aoc_2024::read_input;


fn main() {
    part1();
    part2();
}

fn part1() {
    let mut values = read_values();
    values = repeated_update_values(values, 25);

    let total: usize = values.values().sum();

    // 190865 (but only 507 distinct values i.e. each value has average of ~400 repetitions)
    println!("Total: {total}, distinct values {}", values.len());
}

fn part2() {
    let mut values = read_values();
    values = repeated_update_values(values, 75);

    let total: usize = values.values().sum();

    // 225404711855335 (but only 3782 distinct values)
    println!("Total: {total}, distinct values {}", values.len());
}

fn repeated_update_values(mut values: Values, count: usize) -> Values {
    for _ in 0..count {
        values = update_values(&values);
    }
    values
}

fn update_values(values: &Values) -> Values {
    let mut new_values = Values::new();
    for (&value, &count) in values {
        if value == 0 {
            counting_insert(&mut new_values, 1, count);
        } else if value.to_string().len() % 2 == 0 {
            let s = value.to_string();
            let mid = s.len() / 2;
            let (fst, snd) = (&s[..mid], &s[mid..]);
            counting_insert(&mut new_values, fst.parse().unwrap(), count);
            counting_insert(&mut new_values, snd.parse().unwrap(), count);
        } else {
            counting_insert(&mut new_values, value * 2024, count);
        }
    }
    new_values
}

type Values = HashMap<u64, usize>;

fn counting_insert(values: &mut Values, value: u64, count: usize) {
    values.entry(value).and_modify(|c| *c += count).or_insert(count);
}

fn read_values() -> Values {
    let mut values = Values::new();
    for value in read_input(11).next().unwrap().split_ascii_whitespace()
        .map(|s| s.parse().unwrap()) {
        counting_insert(&mut values, value, 1);
    }
    values
}