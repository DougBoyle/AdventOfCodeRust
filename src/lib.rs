
use std::{
    fs::File,
    io::{BufRead, BufReader}
};

pub mod point;
pub mod direction;

pub fn read_input(day: u32) -> impl Iterator<Item=String> {
    let filename = format!("resources/day{day}.txt");
    let f = File::open(&filename).expect(format!("Couldn't open {filename}").as_str());
    BufReader::new(f).lines().map(Result::unwrap)
}

pub fn split_in_two(s: &str, separator: char) -> (&str, &str) {
    let split: Vec<_> = s.split(separator).collect();
    assert!(split.len() == 2);
    (split[0], split[1])
}

pub fn assert_single<T, I: Iterator<Item=T>>(it: I) -> T {
    let items: Vec<_> = it.collect();
    assert!(items.len() == 1);
    items.into_iter().next().unwrap()
}
