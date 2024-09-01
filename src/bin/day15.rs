use std::{cmp::{max, Ordering}, collections::HashMap, hash::{DefaultHasher, Hash, Hasher}};

use linked_hash_map::LinkedHashMap;
use rust_aoc::{direction::Direction, point::Point};

fn main() {
    let line = rust_aoc::read_input(15).next().unwrap();
    let total: usize = line.split(',').map(hash).sum();
    println!("Part 1: Total hashes {total}"); // 505379

    let mut boxes: Vec<_> = (0..256).map(|_| LinkedHashMap::new()).collect();
    for action in line.split(',') {
        if let Some(idx) = action.find('-') {
            let label = String::from(&action[..idx]);
            let hash = hash(&label);
            boxes[hash].remove(&label);
        } else if let Some(idx) = action.find('=') {
            let label = String::from(&action[..idx]);
            let hash = hash(&label);
            let lens: usize = action[(idx + 1)..].parse().unwrap();
            boxes[hash].entry(label).and_modify(|stored_lens| *stored_lens = lens).or_insert(lens);
        } else {
            panic!("Invalid action {action}");
        }
    }

    let power: usize = boxes.iter().enumerate().map(|(box_num, lenses)| {
        let box_num = box_num + 1;
        box_num * lenses.values().enumerate().map(|(slot, lens)| (slot + 1) * lens).sum::<usize>()
    }).sum();

    println!("Part 2: Total power {power}"); // 263211
}

fn hash(s: &str) -> usize {
    let mut value = 0;
    for b in s.bytes() {
        value += b as usize;
        value *= 17;
        value %= 256;
    }
    value
}
