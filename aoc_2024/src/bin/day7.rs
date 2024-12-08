use aoc_2024::read_input;
use rust_aoc::split_in_two;


fn main() {
    part1();
    part2();
}

fn part1() {
    let total = sum_possible::<Part1>(parse_input());
    println!("Total: {total}"); // 66343330034722
}

fn part2() {
    let total = sum_possible::<Part2>(parse_input());
    println!("Total: {total}"); // 637696070419031
}

fn sum_possible<Ops: Operators>(input: Vec<(u64, Vec<u64>)>) -> u64 {
    input.iter()
        .filter(|(target, values)| is_possible::<Ops>(*target, values))
        .map(|(target, _)| target)
        .sum()
}

fn is_possible<Ops: Operators>(target: u64, values: &Vec<u64>) -> bool {
    let mut it = values.iter();
    match it.next() {
        Some(v) => is_possible_from::<Ops>(*v, target, it),
        None => false,
    }
}

fn is_possible_from<Ops: Operators>(value: u64, target: u64, mut remaining: std::slice::Iter<'_, u64>) -> bool {
    if value > target { return false };
    if let Some(next) = remaining.next() {
        Ops::new_values(value, *next).any(|new_value| is_possible_from::<Ops>(new_value, target, remaining.clone()))
    } else {
        value == target
    }
}

fn parse_input() -> Vec<(u64, Vec<u64>)> {
    read_input(7).map(|s| {
        let (target, values) = split_in_two(&s, ':');
        let target = target.parse().unwrap();
        let values = values.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
        (target, values)
    }).collect()
}

trait Operators {
    fn new_values(x: u64, y: u64) -> impl Iterator<Item=u64>;
}

struct Part1;
impl Operators for Part1 {
    fn new_values(x: u64, y: u64) -> impl Iterator<Item=u64> {
        [x+y, x*y].into_iter()
    }
}

struct Part2;
impl Operators for Part2 {
    fn new_values(x: u64, y: u64) -> impl Iterator<Item=u64> {
        [x+y, x*y, (x.to_string() + &y.to_string()).parse().unwrap()].into_iter()
    }
}
