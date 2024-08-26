
use std::{
    fs::File,
    io::{BufRead, BufReader}
};

pub mod point;

pub fn read_input(day: u32) -> impl Iterator<Item=String> {
    let filename = format!("resources/day{day}.txt");
    let f = File::open(&filename).expect(format!("Couldn't open {filename}").as_str());
    BufReader::new(f).lines().map(Result::unwrap)
}
