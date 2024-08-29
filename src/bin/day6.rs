use std::cmp::Ordering;

fn main() {
    Part1::process();
    Part2::process();
}

struct Part1 {}

impl Part1 {
    fn process() {
        let mut lines = rust_aoc::read_input(6);
        let times = Part1::parse_line(&lines.next().unwrap());
        let distances = Part1::parse_line(&lines.next().unwrap());
        let total_ways: usize = times.iter().zip(distances.iter())
            .map(|(time, distance)| ways_to_beat_distance(*time, *distance))
            .fold(1, |a, b| a*b);
        println!("Total: {total_ways}"); // 160816
    }

    fn parse_line(s: &str) -> Vec<usize> {
        let (_, values) = rust_aoc::split_in_two(s, ':');
        values.split_ascii_whitespace().map(str::parse).map(Result::unwrap).collect()
    }
}

struct Part2 {}

impl Part2 {
    fn process() {
        let mut lines = rust_aoc::read_input(6);
        let time = Part2::parse_line(&lines.next().unwrap());
        let distance = Part2::parse_line(&lines.next().unwrap());
        let possibilities = ways_to_beat_distance(time, distance);
        println!("Total: {possibilities}"); // 46561107
    }

    fn parse_line(s: &str) -> usize {
        let (_, values) = rust_aoc::split_in_two(s, ':');
        let num = values.split_ascii_whitespace().fold(String::new(), |s1, s2| s1 + s2);
        num.parse().unwrap()
    }
}

fn ways_to_beat_distance(time: usize, distance: usize) -> usize {
    let score_to_reach = distance + 1; // need to beat the record
    let mid = time / 2;
    let minimum = search(1, mid, |t| (t*(time-t)).cmp(&score_to_reach));
    let maximum = time - minimum; // problem is symmetric
    (maximum + 1) - minimum
}

/// Returns a value evaluating to Equal, or the index after the last value evaulating to Less
/// if no values returning Equal could be found.
fn search<F: Fn(usize) -> Ordering>(mut start: usize, mut end: usize, f: F) -> usize {
    while end > start {
        let mid = (start + end) / 2;
        match f(mid) {
            Ordering::Less => start = mid + 1,
            Ordering::Equal => return mid,
            Ordering::Greater => end = mid
        }
    }
    start
}
