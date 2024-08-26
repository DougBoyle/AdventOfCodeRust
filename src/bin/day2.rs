use std::str::FromStr;

use rust_aoc::split_in_two;

use enum_map::{enum_map, Enum, EnumMap};

fn main() {
    let part = Part2::new();

    let games = rust_aoc::read_input(2).map(|line| line.parse().unwrap());
    let total = part.process(games);

    // 1. 2512
    // 2. 67335
    println!("Total: {total}");
}

#[derive(Enum, Copy, Clone)]
enum Color { Red, Green, Blue }

impl FromStr for Color {
    type Err = String;

    fn from_str(str: &str) -> Result<Color,Self::Err> {
        match str {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(format!("Unknown color {str}"))
        }
    }
}

trait Part {
    fn new() -> Self;
    fn process<I: Iterator<Item=Game>>(&self, games: I) -> u32;
}

#[allow(dead_code)]
struct Part1 {
    available_cubes: EnumMap<Color, u32>
}

impl Part1 {
    #[allow(dead_code)]
    fn possible(&self, game: &Game) -> bool {
        for (key, value) in game.cubes_seen {
            if self.available_cubes[key] < value { return false }
        }
        true
    }
}

impl Part for Part1 {
    fn new() -> Part1 {
        let available_cubes = enum_map! { Color::Red => 12, Color::Green => 13, Color::Blue => 14 };
        Part1 { available_cubes }
    }

    fn process<I: Iterator<Item=Game>>(&self, games: I) -> u32 {
        games.filter(|game| self.possible(game))
        .map(|game| game.id)
        .sum()
    }
}

struct Game {
    id: u32, cubes_seen: EnumMap<Color, u32>
}

impl FromStr for Game {
    type Err = ();

    fn from_str(str: &str) -> Result<Game,()> {
        let (game, rounds) = Game::split_game_and_rounds(str);
        let id = Game::parse_game_id(game);
        let mut cubes_seen = enum_map! { Color::Red => 0, Color::Green => 0, Color::Blue => 0 };
        Game::split_rounds(rounds).for_each(|round| {
            Game::split_count_and_colors(round).for_each(|(count, color)| {
                if count > cubes_seen[color] { cubes_seen[color] = count }
            })
        });
        Ok(Game { id, cubes_seen })
    }
}

impl Game {
    fn split_game_and_rounds(str: &str) -> (&str, &str) {
        split_in_two(str, ':')
    }

    // Expects 'Game N'
    fn parse_game_id(str: &str) -> u32 {
        let split: Vec<_> = str.split_ascii_whitespace().collect();
        assert!(split.len() == 2);
        split[1].parse().unwrap()
    }

    fn split_rounds(rounds: &str) -> impl std::iter::Iterator<Item=&str> {
        rounds.split(';').map(str::trim)
    }

    fn split_count_and_colors(round: &str) -> impl std::iter::Iterator<Item=(u32, Color)> + '_ {
        round.split(',').map(Game::parse_count_and_color)
    }

    // Expects 'n green'
    fn parse_count_and_color(count_and_color: &str) -> (u32, Color) {
        let (count, color) = split_in_two(count_and_color.trim(), ' ');
        let count = count.parse().unwrap();
        let color = color.parse().unwrap();
        (count, color)
    }
}

struct Part2;

impl Part for Part2 {
    fn new() -> Self {
        Part2 {}
    }

    fn process<I: Iterator<Item=Game>>(&self, games: I) -> u32 {
        games.map(|game| game.cubes_seen)
        .map(|cubes| cubes[Color::Red] * cubes[Color::Blue] * cubes[Color::Green])
        .sum()
    }
}
