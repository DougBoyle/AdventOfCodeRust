use std::iter::{self, zip};

fn main() {
    part1();
    part2();
}

fn part1() {
    let (mut l1, mut l2) = read_input();
    l1.sort();
    l2.sort();
    let total: u32 = zip(l1, l2).map(|(n1, n2)| n1.abs_diff(n2)).sum();

    println!("Total: {total}"); // 2192892
}

fn part2() {
    let (l1, l2) = read_input();

    let mut i1 = sorted_occurrences(l1).peekable();
    let mut i2 = sorted_occurrences(l2).peekable();

    let mut total = 0;
    while let (Some((v1, c1)), Some((v2, c2))) = (i1.peek(), i2.peek()) {
        match v1.cmp(v2) {
            std::cmp::Ordering::Less => { i1.next(); },
            std::cmp::Ordering::Greater => { i2.next(); },
            std::cmp::Ordering::Equal => {
                total += v1 * c1 * c2;
                i1.next();
                i2.next();
            }
        }
    }

    println!("Total: {total}"); // 22962826
}

fn read_input() -> (Vec<u32>, Vec<u32>) {
    aoc_2024::read_input(1)
    .map(|line| {
        let split: Vec<_> = line.split_ascii_whitespace().collect();
        assert!(split.len() == 2);
        (split[0].parse::<u32>().unwrap(), split[1].parse::<u32>().unwrap())
    })
    .unzip()
}

fn sorted_occurrences(mut list: Vec<u32>) -> impl Iterator<Item=(u32, u32)> {
    list.sort();
    let mut it = list.into_iter().peekable();
    iter::from_fn(move || {
        if let Some(value) = it.next() {
            let mut count = 1;
            while it.peek() == Some(&value) {
                it.next();
                count += 1;
            }
            Some((value, count))
        } else {
            None
        }
    })
}