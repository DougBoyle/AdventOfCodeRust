use std::str::FromStr;

use aoc_2024::read_input;
use regex::{Captures, Match, Regex};
use rust_aoc::point::Point;

const BUTTON_PATTERN: &str = "Button [AB]: X\\+(\\d+), Y\\+(\\d+)";
const TARGET_PATTERN: &str = "Prize: X=(\\d+), Y=(\\d+)";

const COST_A: usize = 3;
const COST_B: usize = 1;

const PART_2_OFFSET_SIZE: i64 = 10_000_000_000_000;
const PART_2_OFFSET: Point = Point { x: PART_2_OFFSET_SIZE, y: PART_2_OFFSET_SIZE };

fn main() {
    part1();
    part2();
}

fn part1() {
    let total: usize = read_games().iter()
        .filter_map(cheapest_solution)
        .sum();

    println!("Total: {total}"); // 26005
}

fn part2() {
    let total: usize = read_games().into_iter()
        .map(|game @ Game { target, .. }| Game { target: target + PART_2_OFFSET, ..game})
        .filter_map(|game| cheapest_solution(&game))
        .sum();

    println!("Total: {total}"); // 105620095782547
}

/// Can represent as a matrix:
/// M = [ button_a button_b ]
/// x = [ count_a ]
///     [ count_b ]
/// y = target
/// 
/// M x = y
/// x = M^-1 y
/// (You get the same answer solving a pair of equations normally, which is how you derive the inverse of a 2x2 matrix)
fn cheapest_solution(Game { button_a, button_b, target }: &Game) -> Option<usize> {
    assert!(
        button_a.x > 0 && button_a.y > 0 && button_b.x > 0 && button_b.y > 0,
        "Simplified logic assumes all coordniates > 0: A={button_a}, B={button_b}"
    );

    let determinant = button_a.x * button_b.y - button_a.y * button_b.x;
    assert!(determinant != 0, "Need to handle 0 determinant i.e. parallel button directions, reduces to 1D problem with possibly many solutions");

    let count_a = (button_b.y * target.x - button_b.x * target.y) / determinant;
    let count_b = (-button_a.y * target.x + button_a.x * target.y) / determinant;

    if count_a >= 0 && count_b >= 0 && (count_a * *button_a) + (count_b * *button_b) == *target {
        return Some((count_a * COST_A as i64 + count_b * COST_B as i64) as usize)
    } else {
        return None
    }
}

struct Game {
    button_a: Point,
    button_b: Point,
    target: Point,
}

fn read_games() -> Vec<Game> {
    let button_pattern = Regex::new(BUTTON_PATTERN).unwrap();
    let target_pattern = Regex::new(TARGET_PATTERN).unwrap();

    let mut result = Vec::new();
    let mut it = read_input(13);
    loop {
        result.push(read_game(&mut it, &button_pattern, &target_pattern));
        if let None = it.next() { return result } // skip blank line between games
    }
}

fn read_game(it: &mut impl Iterator<Item=String>, button_pattern: &Regex, target_pattern: &Regex) -> Game {
    let button_a = it.next().unwrap();
    let button_b = it.next().unwrap();
    let target = it.next().unwrap();

    let button_a = button_pattern.captures(&button_a).unwrap();
    let button_b = button_pattern.captures(&button_b).unwrap();
    let target = target_pattern.captures(&target).unwrap();

    let button_a = point_from_capture(button_a);
    let button_b = point_from_capture(button_b);
    let target = point_from_capture(target);

    Game { button_a, button_b, target }
}

/// The provided capture should have 2 groups for the X then Y coordinates
fn point_from_capture(captures: Captures) -> Point {
    let x = parse(captures.get(1).unwrap()).unwrap();
    let y = parse(captures.get(2).unwrap()).unwrap();
    Point { x, y }
}

fn parse<T: FromStr>(m: Match) -> Result<T, T::Err> {
    m.as_str().parse()
}
