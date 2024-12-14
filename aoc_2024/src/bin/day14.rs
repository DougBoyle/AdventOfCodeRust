use std::{io::Error, str::FromStr, sync::LazyLock, thread, time::Duration};

use aoc_2024::read_input;
use regex::{Captures, Regex};
use rust_aoc::{grid::Grid, point::Point};

const LINE_PATTERN: &str = "p=(-?\\d+),(-?\\d+) v=(-?\\d+),(-?\\d+)";
static LINE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(LINE_PATTERN).unwrap());

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut quadrants = vec![0, 0, 0, 0]; // order of quadrants not meaningful
    for Point { x, y } in read_paths().into_iter()
        .map(|Path { position, velocity }| position + 100 * velocity)
        .map(|Point { x, y }| Point { x: x.rem_euclid(WIDTH), y: y.rem_euclid(HEIGHT) }) {
        if x > WIDTH / 2 {
            if y > HEIGHT / 2 {
                quadrants[0] += 1;
            } else if y < HEIGHT / 2 {
                quadrants[1] += 1;
            }
        } else if x < WIDTH / 2 {
            if y > HEIGHT / 2 {
                quadrants[2] += 1;
            } else if y < HEIGHT / 2 {
                quadrants[3] += 1;
            }
        }
    }

    let total = quadrants.into_iter().fold(1, |x, y| x*y);

    println!("Total: {total}"); // 229868730
}

fn part2() {
    let mut paths = read_paths();
    for i in 0..=7861 {
        // Print
        let mut output = format!("================================== Iteration {i} ==================================\n");
        let mut grid = Grid::generate(WIDTH, HEIGHT, &mut |_| false);
        for Path { position, .. } in paths.iter() {
            grid[position] = true;
        }
        for row in grid.iter_rows() {
            output += "|";
            for cell in row {
                output += if *cell { "#" } else { " " };
            }
            output += "|\n";
        }
        println!("{output}");
        // Wait
        let distance = 7861i32.abs_diff(i);
        let sleep = if distance > 1000 {
            1
        } else if distance > 100 {
            10
        } else if distance > 30 {
            100
        } else if distance > 10 {
            200
        } else if distance > 4 {
            500
        } else {
            1000
        };
        thread::sleep(Duration::from_millis(sleep));
        // Update
        for path in paths.iter_mut() {
            let Point { x, y } = path.position + path.velocity;
            path.position = Point { x: x.rem_euclid(WIDTH), y: y.rem_euclid(HEIGHT) }
        }
    }

    // Horizontal pattern: iteration 1063
    // - require at least as many in row to print:
    //   '##  #   # #     # ## # #  #    #   #  #  #    #   #  #        ##      #    ##    #  #  #   #  #'
    // - at least 25 (500 in total, so 5%)
    // Solution: 7861
}

struct Path {
    position: Point,
    velocity: Point,
}

impl FromStr for Path {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = LINE_RE.captures(s).expect(&format!("Couldn't match '{s}'"));
        let x = parse_match(&captures, 1);
        let y = parse_match(&captures, 2);
        let vx = parse_match(&captures, 3);
        let vy = parse_match(&captures, 4);
        Ok(Path { position: Point { x, y }, velocity: Point { x: vx, y: vy } })
    }
}

fn read_paths() -> Vec<Path> {
    read_input(14).map(|s| s.parse().unwrap()).collect()
}

fn parse_match<T: FromStr>(captures: &Captures, i: usize) -> T
where <T as FromStr>::Err: std::fmt::Debug {
    captures.get(i).unwrap().as_str().parse().unwrap()
}
