use std::collections::{HashMap, HashSet};

use aoc_2024::read_input;
use rust_aoc::{grid::Grid, point::Point};


fn main() {
    part1();
    part2();
}

fn part1() {
    let grid = Grid::parse(read_input(8), |c| c);
    let mut antennas = HashMap::new();
    let mut antinodes = HashSet::new();
    for (p, freq) in enumerate_antennas(&grid) {
        let freq_antennas = antennas.entry(freq).or_insert(vec![]);
        for p2 in freq_antennas.iter() {
            let delta = *p2 - p;
            let node1 = *p2 + delta;
            let node2 = p - delta;
            if grid.is_in_bounds(&node1) { antinodes.insert(node1); }
            if grid.is_in_bounds(&node2) { antinodes.insert(node2); }
        }
        freq_antennas.push(p);
    }

    println!("Total: {}", antinodes.len()); // 293
}

fn part2() {
    let grid = Grid::parse(read_input(8), |c| c);
    let mut antennas = HashMap::new();
    let mut antinodes = HashSet::new();
    for (p, freq) in enumerate_antennas(&grid) {
        let freq_antennas = antennas.entry(freq).or_insert(vec![]);
        for p2 in freq_antennas.iter() {
            let delta = *p2 - p;
            antinodes.insert(p);
            antinodes.insert(*p2);
            let mut node = *p2 + delta;
            while grid.is_in_bounds(&node) {
                antinodes.insert(node);
                node += delta;
            }
            let mut node = p - delta;
            while grid.is_in_bounds(&node) {
                antinodes.insert(node);
                node -= delta;
            }
        }
        freq_antennas.push(p);
    }

    println!("Total: {}", antinodes.len()); // 934
}

fn enumerate_antennas(grid: &Grid<char>) -> impl Iterator<Item=(Point, &char)> {
    grid.enumerate().filter(|(_, freq)| **freq != '.')
}

