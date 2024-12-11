use std::collections::BTreeMap;

use aoc_2024::read_input;


fn main() {
    part1();
    part2();
}

const EMPTY: i64 = -1;

fn part1() {
    let digits = read_digits();

    let mut disk = Vec::new();
    let mut file_id = 0;
    let mut is_file = true;

    for len in digits {
        for _ in 0..len {
            disk.push(if is_file { file_id } else { EMPTY });
        }
        if is_file {
            file_id += 1;
            is_file = false;
        } else {
            is_file = true;
        }
    }

    let mut start = 0;
    let mut end = disk.len() - 1;
    while start < end {
        if disk[start] != EMPTY {
            start += 1;
        } else if disk[end] == EMPTY {
            end -= 1;
        } else {
            disk.swap(start, end);
            start += 1;
            end -= 1;
        }
    }

    let total: i64 = disk.into_iter().enumerate()
        .filter(|(_, value)| *value != EMPTY)
        .map(|(i, value)| i as i64 * value)
        .sum();

    println!("Total: {total}"); // 6341711060162
}

struct File {
    file_id: i64,
    len: i64,
}

fn part2() {
    let digits = read_digits();

    let mut files_by_start_pos = BTreeMap::new();
    let mut position = 0;
    let mut next_file_id = 0;
    let mut is_file = true;

    for len in digits {
        if is_file {
            files_by_start_pos.insert(position, File { file_id: next_file_id, len });
            next_file_id += 1;
            is_file = false;
        } else {
            is_file = true;
        }
        position += len;
    }

    let mut file_to_move = next_file_id - 1;
    let mut last_file_pos = position;
    while file_to_move > 0 {
        // scan backwards for th next file to move
        let (&pos, file) = files_by_start_pos.range(..last_file_pos).last().unwrap();
        last_file_pos = pos;
        if file.file_id != file_to_move { continue; }

        // scan forwards for somewhere to put it
        let mut last_file_end = 0;
        let move_to = files_by_start_pos.range(..=pos).filter_map(|(start, File { len, .. })| {
            let gap = start - last_file_end;
            if gap >= file.len {
                // can move here
                Some(last_file_end)
            } else {
                last_file_end = start + len;
                None
            }
        }).next();

        if let Some(move_to) = move_to {
            let file = files_by_start_pos.remove(&pos).unwrap();
            files_by_start_pos.insert(move_to, file);
        }

        file_to_move -= 1;
    }

    let total: i64 = files_by_start_pos.into_iter()
        .map(|(start, File { file_id, len })| (start..start + len).sum::<i64>() * file_id)
        .sum();

    println!("Total: {total}"); // 6377400869326
}

fn read_digits() -> Vec<i64> {
    read_input(9).next().unwrap().chars().map(|c| c.to_digit(10).unwrap() as i64).collect()
}