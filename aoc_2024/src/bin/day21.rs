use std::fmt::Debug;
use std::iter::repeat;

use aoc_2024::read_input;
use rust_aoc::{direction::Direction, point::Point, MultiHashSet};

fn main() {
    assert_should_do_furthest_button_first();
    part1();
    part2();
}

fn part1() {
    let codes = parse_input();
    let total: usize = codes.into_iter().map(|code| get_complexity(code, 2)).sum();
    println!("Total {total}"); // 137870
}

fn part2() {
    let codes = parse_input();
    let total: usize = codes.into_iter().map(|code| get_complexity(code, 25)).sum();
    println!("Total {total}"); // 170279148659464
}

fn get_code_value(code: &Vec<NumericInput>) -> u32 {
    code.iter().fold(0, |acc, value| match value.0.to_digit(10) {
        None => acc,
        Some(n) => 10*acc + n
    })
}

fn get_complexity(code: Vec<NumericInput>, indirections: usize) -> usize {
    let mut directions = directions_to_type_input(&code);
    for _ in 0..indirections {
        directions = directions_to_type_all_inputs(&directions);
    }
    let code_value = get_code_value(&code);
    code_value as usize * directions.len()
}

fn directions_to_type_all_inputs(input: &Segments) -> Segments {
    let mut result = Segments::new();
    for (segment, count) in input.iter() {
        for (new_segment, repetitions) in directions_to_type_input(segment).into_iter() {
            result.insert_many(new_segment, count * repetitions);
        }
    }
    result
}

fn directions_to_type_input<T: Keypad>(input: &[T]) -> Segments {
    let mut directions = Segments::new();
    let mut position = T::start().position();
    for button in input {
        let new_position = button.position();
        let offset = new_position - position;
        let mut segment = as_directions::<T>(position, offset);
        segment.push(DirectionInput::Activate);
        directions.insert(segment);
        position = new_position;
    }
    directions
}

/// For an optimal sequence, go all the way in one direction, then all the way in the other.
/// Need to avoid the empty corner tile of the keypad, but otherwise prefer the direction with
/// the further away button first, see assert_should_do_furthest_button_first for why.
fn as_directions<T: Keypad>(current_position: Point, offset: Point) -> Vec<DirectionInput> {
    let x_steps = x_axis_steps(offset.x);
    let y_steps = y_axis_steps(offset.y);
    if x_steps.is_empty() {
        y_steps
    } else if y_steps.is_empty() {
        x_steps
    } else {
        let x_first_position = Point { x: current_position.x + offset.x, ..current_position };
        let y_first_position = Point { y: current_position.y + offset.y, ..current_position };
        if !T::from_position(x_first_position).is_some() { // x first would go off keypad
            let mut result = y_steps;
            result.extend(x_steps.into_iter());
            result
        } else if !T::from_position(y_first_position).is_some() { // y first would go off keypad
            let mut result = x_steps.clone();
            result.extend(y_steps.iter().cloned());
            result
        } else {
            // choose the direction with the further away button first
            let x_dir_distance = DirectionInput::Activate.position().orthogonal_distance(&x_steps[0].position());
            let y_dir_distance = DirectionInput::Activate.position().orthogonal_distance(&y_steps[0].position());
            if x_dir_distance > y_dir_distance {
                let mut result = x_steps.clone();
                result.extend(y_steps.iter().cloned());
                result
            } else {
                let mut result = y_steps;
                result.extend(x_steps.into_iter());
                result
            }
        }
    }
}

/// Prefer doing the direction further from the Activate button first, to keep the intermediate movement between the
/// buttons cheap, rather than repeating the slower movements after returning to Activate. Note that the first movement
/// is never "cheap", given you either have to go LEFT or DOWN from the Activate button, and getting back to Activate
/// having pressed both directions is always cheap, as it is just RIGHT and UP presses. As such, the only thing to
/// optimise is the buttons required for the intermediate step between the two buttons.
///
/// Left/Up:
/// If we need to move e.g. x-2, y-1, we have a choice of pressing LEFT first or UP.
/// Should always do LEFT first, as at a higher level of indirection moving left is costly (3 steps from activate button),
/// and we only need to do this once if we press the leftmost button (LEFT) first, then move right to the other.
/// 
/// To press both LEFT and UP, we will move left to either the LEFT button or UP, and then back to Activate to press it.
/// After that, we either need to move left again from UP -> LEFT, or right to go from LEFT -> UP.
/// At the higher level of indirection, the RIGHT button is nearer, so pressing LEFT first and then UP is cheaper.
/// 
/// Right/Down:
/// Similarly, if we need to move both DOWN and RIGHT, press DOWN first.
/// After moving to DOWN at the bottom level, and back to Activate at the higher level in order to press it, moving
/// DOWN -> RIGHT just requires a RIGHT press, which is 1 away from Activate. Had we pressed RIGHT first, now need a LEFT
/// press to get to DOWN, which at the higher level is further from the Activate button we had to return to in between.
/// 
/// ================================ Alternative Approach (Dynamic Programming) ================================
/// 
/// Problem with working up from the final number pad is that it's hard to tell how sequences will grow as we get more
/// abstract, in order to identify which of several possible paths is actually shortest at the top (see above approach).
/// Working down from the top most abstract directional pad also isn't straightforward, as we don't know which paths
/// to consider to get the right sequence at the final number pad.
/// 
/// This can be solved by Dynamic Programming, remembering the length (in terms of top-level button presses) of the
/// shortest sequence to go from every button X to every Y and press it, at each depth starting from the most abstract.
/// These can then be combined to determine the shortest way to get between each pair of buttons at the next level.
/// As usual with dynamic programming, rather than actually calculating all of these up front, they can be worked out
/// on demand and stored in a cache based on the queries actually required to solve the bottom level.
fn assert_should_do_furthest_button_first() {
    for x_dir in [DirectionInput::West, DirectionInput::East] {
        for y_dir in [DirectionInput::North, DirectionInput::South] {
            let steps = [x_dir, y_dir, DirectionInput::Activate];
            let directions = directions_to_type_input(&steps);
            let directions = directions_to_type_all_inputs(&directions);
            let x_first_steps = directions.len();

            let steps = [y_dir, x_dir, DirectionInput::Activate];
            let directions = directions_to_type_input(&steps);
            let directions = directions_to_type_all_inputs(&directions);
            let y_first_steps = directions.len();

            let x_dir_distance = DirectionInput::Activate.position().orthogonal_distance(&x_dir.position());
            let y_dir_distance = DirectionInput::Activate.position().orthogonal_distance(&y_dir.position());

            match x_dir_distance.cmp(&y_dir_distance) {
                std::cmp::Ordering::Less => assert!(y_first_steps < x_first_steps),
                std::cmp::Ordering::Equal => assert_eq!(x_first_steps, y_first_steps),
                std::cmp::Ordering::Greater => assert!(x_first_steps < y_first_steps),
            }
        }
    }
}


fn x_axis_steps(offset: i64) -> Vec<DirectionInput> {
    if offset > 0 {
        repeat(DirectionInput::East).take(offset as usize).collect()
    } else {
        repeat(DirectionInput::West).take(offset.abs() as usize).collect()
    }
}

fn y_axis_steps(offset: i64) -> Vec<DirectionInput> {
    if offset > 0 {
        repeat(DirectionInput::South).take(offset as usize).collect()
    } else {
        repeat(DirectionInput::North).take(offset.abs() as usize).collect()
    }
}

struct Segments(MultiHashSet<Vec<DirectionInput>>);

impl Segments {
    fn new() -> Self {
        Segments(MultiHashSet::new())
    }

    fn iter(&self) -> impl Iterator<Item=(&Vec<DirectionInput>, &usize)> {
        self.0.iter()
    }

    fn into_iter(self) -> impl Iterator<Item=(Vec<DirectionInput>, usize)> {
        self.0.into_iter()
    }

    fn len(&self) -> usize {
        self.iter().map(|(segment, count)| segment.len() * count).sum()
    }

    fn insert(&mut self, segment: Vec<DirectionInput>) {
        self.0.insert(segment);
    }

    fn insert_many(&mut self, segment: Vec<DirectionInput>, count: usize) {
        self.0.insert_many(segment, count);
    }
}

trait Keypad: Sized {
    fn start() -> Self;
    /// Same as Direction enum, y = 0 is at the top, not the bottom.
    fn position(&self) -> Point;
    fn from_position(point: Point) -> Option<Self>;
}

const ACTIVATE: char = 'A';

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct NumericInput(char);

impl Keypad for NumericInput {
    fn start() -> Self {
        NumericInput(ACTIVATE)
    }

    fn position(&self) -> Point {
        match self.0 {
            '7' => Point { x: 0, y: 0 },
            '8' => Point { x: 1, y: 0 },
            '9' => Point { x: 2, y: 0 },
            '4' => Point { x: 0, y: 1 },
            '5' => Point { x: 1, y: 1 },
            '6' => Point { x: 2, y: 1 },
            '1' => Point { x: 0, y: 2 },
            '2' => Point { x: 1, y: 2 },
            '3' => Point { x: 2, y: 2 },
            '0' => Point { x: 1, y: 3 },
            ACTIVATE => Point { x: 2, y: 3 },
            _ => panic!("Invalid input {}", self.0),
        }
    }

    fn from_position(point: Point) -> Option<NumericInput> {
        Some(NumericInput(match point {
            Point { x: 0, y: 0 } => '7',
            Point { x: 1, y: 0 } => '8',
            Point { x: 2, y: 0 } => '9',
            Point { x: 0, y: 1 } => '4',
            Point { x: 1, y: 1 } => '5',
            Point { x: 2, y: 1 } => '6',
            Point { x: 0, y: 2 } => '1',
            Point { x: 1, y: 2 } => '2',
            Point { x: 2, y: 2 } => '3',
            Point { x: 1, y: 3 } => '0',
            Point { x: 2, y: 3 } => ACTIVATE,
            _ => None?,
        }))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum DirectionInput {
    North, East, South, West, Activate
}

impl From<Direction> for DirectionInput {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => DirectionInput::North,
            Direction::East => DirectionInput::East,
            Direction::South => DirectionInput::South,
            Direction::West => DirectionInput::West,
        }
    }
}

impl From<DirectionInput> for char {
    fn from(value: DirectionInput) -> Self {
        match value {
            DirectionInput::North => '^',
            DirectionInput::East => '>',
            DirectionInput::South => 'v',
            DirectionInput::West => '<',
            DirectionInput::Activate => 'A',
        }
    }
}

impl From<char> for DirectionInput {
    fn from(value: char) -> Self {
        match value {
            '^' => DirectionInput::North,
            '>' => DirectionInput::East,
            'v' => DirectionInput::South,
            '<' => DirectionInput::West,
            'A' => DirectionInput::Activate,
            _ => panic!("Unrecognised direction input {value}")
        }
    }
}

impl Keypad for DirectionInput {
    fn start() -> Self {
        DirectionInput::Activate
    }

    fn position(&self) -> Point {
        match self {
            DirectionInput::North => Point { x: 1, y: 0 },
            DirectionInput::East => Point { x: 2, y: 1 },
            DirectionInput::South => Point { x: 1, y: 1 },
            DirectionInput::West => Point { x: 0, y: 1 },
            DirectionInput::Activate => Point { x: 2, y: 0 },
        }
    }

    fn from_position(point: Point) -> Option<DirectionInput> {
        Some(match point {
            Point { x: 1, y: 0 } => DirectionInput::North,
            Point { x: 2, y: 1 } => DirectionInput::East,
            Point { x: 1, y: 1 } => DirectionInput::South,
            Point { x: 0, y: 1 } => DirectionInput::West,
            Point { x: 2, y: 0 } => DirectionInput::Activate,
            _ => None?,
        })
    }
}

fn parse_input() -> Vec<Vec<NumericInput>> {
    read_input(21).map(|line| line.chars().map(NumericInput).collect()).collect()
}
