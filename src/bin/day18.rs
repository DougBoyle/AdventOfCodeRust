use rust_aoc::{direction::Direction, point::Point};

fn main() {
    let instructions: Vec<_> = rust_aoc::read_input(18).map(|s| parse_line(&s)).collect();

    let area =  calc_enclosed_cells(instructions.iter().map(|(dir, steps, _)| (*dir, *steps)));
    println!("Part 1: Area {area}"); // 41019

    let area =  calc_enclosed_cells(instructions.iter().map(|(_, _, colour_instructions)| *colour_instructions));
    println!("Part 2: Area {area}"); // 96116995735219
}

fn calc_enclosed_cells(instructions: impl Iterator<Item=(Direction, i32)>) -> i64 {
    let mut position = Point {x: 0, y: 0};
    let mut border = vec![position];
    for (dir, steps) in instructions {
        let movement = Point::from(dir) * steps;
        position += movement;
        border.push(position);
    }
    rust_aoc::shoelace_area_enclosed_cells_including_border(&border)
}

fn parse_line(line: &str) -> (Direction, i32, (Direction, i32)) {
    let [dir, steps, colour] = line.split_ascii_whitespace().collect::<Vec<_>>().try_into().unwrap();
    let dir = parse_direction(dir);
    let steps = steps.parse().unwrap();
    let colour_intructions = parse_colour(colour);
    (dir, steps, colour_intructions)
}

fn parse_direction(dir: &str) -> Direction {
    match dir {
        "R" | "0" => Direction::East,
        "D" | "1" => Direction::South,
        "L" | "2" => Direction::West,
        "U" | "3" => Direction::North,
        _ => panic!("Unrecogniesd direction {dir}")
    }
}

fn parse_colour(colour: &str) -> (Direction, i32) {
    // '(#FF00FF)'
    let steps = i32::from_str_radix(&colour[2..7], 16).unwrap();
    let dir = parse_direction(&colour[7..8]);
    (dir, steps)
}
