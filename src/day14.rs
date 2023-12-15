use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::Vector2D;

#[derive(Debug, Clone)]
struct Platform {
    width: i32,
    height: i32,
    cubes: HashSet<Vector2D>,
    rounds: Vec<Vector2D>,
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Platform {
    let width = input.lines().count() as i32;
    let height = input.lines().next().unwrap().len() as i32;
    let mut cubes = HashSet::new();
    let mut rounds = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '#' => {
                    cubes.insert(Vector2D::new(x as i32, y as i32));
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

impl Platform {
    fn roll_north(&mut self) {
        // Note: rounds must be sorted from north to south
        for i in 0..self.rounds.len() {
            let mut round = self.rounds[i];
            loop {
                let next_pos = round + Vector2D::new(0, -1);
                if round.y() > 0
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
    todo!()
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
        assert_eq!(part2(&parse(INPUT)), 0);
    }
}
