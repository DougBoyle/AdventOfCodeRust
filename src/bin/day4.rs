use rust_aoc::split_in_two;

use std::collections::HashSet;

fn main() {
    part1();
    part2();
}

fn part1() {
    let total: u32 = rust_aoc::read_input(4).map(|line| score(&line)).sum();
    println!("Total: {total}") // 21558
}

fn score(line: &str) -> u32 {
    let count = count_winners(&line);

    if count == 0 { 0 } else { 2_u32.pow(count - 1) }
}

fn part2() {
    let num_winners: Vec<_> = rust_aoc::read_input(4).map(|line| count_winners(&line)).collect();
    let len = num_winners.len();
    let mut num_cards = vec![1; len];
    
    for i in 0..len {
        let num_copies = num_cards[i];
        let winners: usize = num_winners[i].try_into().unwrap();
        for j in (i+1)..(i+1+winners) {
            num_cards[j] += num_copies;
        }
    }

    let total: u32 = num_cards.iter().sum();
    println!("Total: {total}"); // 10425665
}

fn count_winners(line: &str) -> u32 {
    let (_, values) = split_in_two(line, ':');
    let (winners, values) = split_in_two(values, '|');

    let winners: HashSet<usize> = parse_numbers_list(&winners).collect();
    
    parse_numbers_list(values)
        .filter(|v| winners.contains(v))
        .count().try_into().unwrap()
}

fn parse_numbers_list(s: &str) -> impl Iterator<Item=usize> + '_ {
    s.split_ascii_whitespace().map(str::parse).map(Result::unwrap)
}
