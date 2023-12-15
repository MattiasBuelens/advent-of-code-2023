use std::cmp::Ordering;
use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::Vector2D;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Platform {
    width: i32,
    height: i32,
    cubes: Vec<Vector2D>,
    rounds: Vec<Vector2D>,
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Platform {
    let width = input.lines().count() as i32;
    let height = input.lines().next().unwrap().len() as i32;
    let mut cubes = Vec::new();
    let mut rounds = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '#' => {
                    cubes.push(Vector2D::new(x as i32, y as i32));
                }
                'O' => {
                    rounds.push(Vector2D::new(x as i32, y as i32));
                }
                _ => panic!("invalid input {c}"),
            };
        }
    }
    Platform {
        width,
        height,
        cubes,
        rounds,
    }
}

fn compare_rounds(left: &Vector2D, right: &Vector2D) -> Ordering {
    left.y()
        .cmp(&right.y())
        .then_with(|| left.x().cmp(&right.x()))
}

impl Platform {
    fn roll_north(&mut self) {
        for i in 0..self.rounds.len() {
            let mut round = self.rounds[i];
            loop {
                let next_pos = round + Vector2D::new(0, -1);
                if next_pos.y() >= 0
                    && !self.cubes.contains(&next_pos)
                    && !self.rounds.contains(&next_pos)
                {
                    round = next_pos;
                } else {
                    break;
                }
            }
            self.rounds[i] = round;
        }
        self.rounds.sort_by(compare_rounds);
    }

    fn roll_south(&mut self) {
        for i in (0..self.rounds.len()).rev() {
            let mut round = self.rounds[i];
            loop {
                let next_pos = round + Vector2D::new(0, 1);
                if next_pos.y() < self.height
                    && !self.cubes.contains(&next_pos)
                    && !self.rounds.contains(&next_pos)
                {
                    round = next_pos;
                } else {
                    break;
                }
            }
            self.rounds[i] = round;
        }
        self.rounds.sort_by(compare_rounds);
    }

    fn roll_west(&mut self) {
        for i in 0..self.rounds.len() {
            let mut round = self.rounds[i];
            loop {
                let next_pos = round + Vector2D::new(-1, 0);
                if next_pos.x() >= 0
                    && !self.cubes.contains(&next_pos)
                    && !self.rounds.contains(&next_pos)
                {
                    round = next_pos;
                } else {
                    break;
                }
            }
            self.rounds[i] = round;
        }
        self.rounds.sort_by(compare_rounds);
    }

    fn roll_east(&mut self) {
        for i in (0..self.rounds.len()).rev() {
            let mut round = self.rounds[i];
            loop {
                let next_pos = round + Vector2D::new(1, 0);
                if next_pos.x() < self.width
                    && !self.cubes.contains(&next_pos)
                    && !self.rounds.contains(&next_pos)
                {
                    round = next_pos;
                } else {
                    break;
                }
            }
            self.rounds[i] = round;
        }
        self.rounds.sort_by(compare_rounds);
    }

    fn spin_cycle(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    fn total_load_north(&self) -> i32 {
        self.rounds
            .iter()
            .map(|round| self.height - round.y())
            .sum()
    }
}

#[aoc(day14, part1)]
fn part1(input: &Platform) -> i32 {
    let mut platform = input.clone();
    platform.roll_north();
    platform.total_load_north()
}

#[aoc(day14, part2)]
fn part2(input: &Platform) -> i32 {
    let target = 1_000_000_000u64;
    let mut platform = input.clone();
    let mut seen = HashMap::new();
    let mut count = 0u64;
    while !seen.contains_key(&platform) {
        seen.insert(platform.clone(), count);
        platform.spin_cycle();
        count += 1;
    }
    let loop_start = *seen.get(&platform).expect("no loop found");
    let loop_length = count - loop_start;
    let target_count = loop_start + ((target - loop_start) % loop_length);
    let (target_platform, _) = seen
        .iter()
        .find(|(_platform, count)| **count == target_count)
        .unwrap_or_else(|| panic!("no platform found at step {target_count}"));
    target_platform.total_load_north()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 136);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 64);
    }
}
