use std::collections::HashSet;

use rust_aoc::point::Point;

fn main() {
    let input = Input::load();
    Part1::process(&input);
    Part2::process(&input);
}

trait Part {
    fn process(input: &Input);
}

struct Input {
    symbol_grid: HashSet<Point>,
    gears: HashSet<Point>,
    numbers: HashSet<GridNumber>
}

impl Input {
    fn load() -> Input {
        let mut symbol_grid = HashSet::new();
        let mut gears = HashSet::new();
        let mut numbers = HashSet::new();

        rust_aoc::read_input(3)
            .enumerate()
            .for_each(|(i, line)| LineParser::parse(&line, i, &mut symbol_grid, &mut gears, &mut numbers));

        Input { symbol_grid, gears, numbers }
    }
}

struct Part1 {}

impl Part for Part1 {
    fn process(input: &Input) {
        let total: u32 = input.numbers.iter()
            .filter(|num| num.is_part(&input.symbol_grid))
            .map(|num| num.value)
            .sum();

        // 1. 531561
        println!("Total: {total}");
    }
}

struct Part2;

impl Part for Part2 {
    fn process(input: &Input) {
        let total: u32 = input.gears.iter()
            .map(|gear| input.numbers.iter().filter(|num| num.touches(gear)).collect::<Vec<_>>())
            .filter(|nums| nums.len() == 2)
            .map(|nums| nums[0].value * nums[1].value)
            .sum();

        // 2. 83279367
        println!("Total: {total}");
    }
}

struct LineParser<'a> {
    y: i64,
    current_num: u32,
    current_num_start: Point,
    symbol_grid: &'a mut HashSet<Point>,
    gears: &'a mut HashSet<Point>, 
    numbers: &'a mut HashSet<GridNumber>
}

impl LineParser<'_> {
    fn parse(
        line: &str,
        y: usize, 
        symbol_grid: &mut HashSet<Point>,
        gears: &mut HashSet<Point>, 
        numbers: &mut HashSet<GridNumber>
    ) {
        let mut parser = LineParser {
            y: y.try_into().unwrap(), 
            current_num: 0, 
            current_num_start: Point {x:0, y:0},
            symbol_grid,
            gears,
            numbers
        };

        for (x, c) in line.chars().enumerate() {
            let x = x.try_into().unwrap();
            match Cell::parse(c) {
                Cell::Digit(n) => parser.process_digit(n, x),
                Cell::Gap => parser.process_gap(x),
                Cell::Symbol => parser.process_symbol(x),
                Cell::Gear => parser.process_gear(x)
            }
        }

        // flush current_num at end of line
        parser.process_gap(line.len().try_into().unwrap());
    }

    fn process_digit(&mut self, n: u32, x: i64) {
        if self.current_num == 0 {
            self.current_num_start = Point { x, y: self.y }
        }
        self.current_num *= 10;
        self.current_num += n;
    }

    fn process_gear(&mut self, x: i64) {
        self.gears.insert(Point {x, y: self.y});
        self.process_symbol(x);
    }

    fn process_symbol(&mut self, x: i64) {
        self.symbol_grid.insert(Point { x, y: self.y });
        self.process_gap(x);
    }

    fn process_gap(&mut self, x: i64) {
        if self.current_num == 0 { return }

        let end = Point { x: x-1, y: self.y };
        let num = GridNumber { start: self.current_num_start, end, value: self.current_num };
        self.numbers.insert(num);

        self.current_num = 0;
    }
}

enum Cell {
    Digit(u32),
    Symbol,
    Gear,
    Gap
}

impl Cell {
    fn parse(c: char) -> Cell {
        if c == '.' { return Cell::Gap }
        if c == '*' { return Cell::Gear }
        match c.to_digit(10) {
            Some(n) => Cell::Digit(n),
            None => Cell::Symbol
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
struct GridNumber { start: Point, end: Point, value: u32 }

impl GridNumber {
    fn is_part(&self, symbol_grid: &HashSet<Point>) -> bool {
        let top_left = self.start + Point { x: -1, y: -1 };
        let top_right = self.end + Point { x: 1, y: -1 };
        for p in top_left.to_inclusive(top_right) {
            if symbol_grid.contains(&p) { return true }
        }

        let bottom_left = self.start + Point { x: -1, y: 1 };
        let bottom_right = self.end + Point { x: 1, y: 1 };
        for p in bottom_left.to_inclusive(bottom_right) {
            if symbol_grid.contains(&p) { return true }
        }

        let left = self.start + Point { x: -1, y: 0 };
        if symbol_grid.contains(&left) { return true }
        let right = self.end + Point { x: 1, y: 0 };
        symbol_grid.contains(&right)
    }

    fn touches(&self, p: &Point) -> bool {
        p.x >= self.start.x - 1
        && p.x <= self.end.x + 1
        && p.y >= self.start.y - 1 
        && p.y <= self.start.y + 1
    }
}

