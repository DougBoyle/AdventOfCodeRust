use aoc_2024::read_input;
use rust_aoc::{direction8::Direction8, grid::Grid, point::Point};


fn main() {
    part1();
    part2();
}

fn part1() {
    let grid = read_grid();
    let total = grid.enumerate()
        .flat_map(|(p, _)| Direction8::all().map(|d| (p, d)))
        .filter(|(p, d)| "XMAS".chars().enumerate().all(|(i, c)| {
            let p = *p + (i as i64 * Point::from(*d));
            grid.is_in_bounds(&p) && grid[&p] == c
        }))
        .count();

    println!("Total: {total}"); // 2575
}

fn part2() {
    let grid = read_grid();
    let total = grid.enumerate()
        .filter(|(p, _)| {
            Direction8::diagonals().into_iter().filter(|d| {
                "MAS".chars().enumerate().all(|(i, c)| {
                    let p = *p + (((i as i64) - 1) * Point::from(*d));
                    grid.is_in_bounds(&p) && grid[&p] == c
                })
            }).count() == 2
        })
        .count();
    println!("Total: {total}"); // 2041
}

fn read_grid() -> Grid<char> {
    Grid::parse(read_input(4), |c| c)
}
