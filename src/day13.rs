use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Pattern {
    width: usize,
    height: usize,
    cells: Vec<Vec<char>>,
}

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();
        let cells = s.lines().map(|line| line.chars().collect()).collect();
        Ok(Self {
            width,
            height,
            cells,
        })
    }
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Pattern> {
    input.split("\n\n").map(|s| s.parse().unwrap()).collect()
}

impl Pattern {
    fn is_vertical_mirror(&self, mirror: usize) -> bool {
        let max_offset = mirror.min(self.width.saturating_sub(mirror));
        for dx in 0..max_offset {
            for y in 0..self.height {
                if self.cells[y][mirror - dx - 1] != self.cells[y][mirror + dx] {
                    return false;
                }
            }
        }
        true
    }

    fn is_horizontal_mirror(&self, mirror: usize) -> bool {
        let max_offset = mirror.min(self.height.saturating_sub(mirror));
        for dy in 0..max_offset {
            if self.cells[mirror - dy - 1] != self.cells[mirror + dy] {
                return false;
            }
        }
        true
    }

    fn find_vertical_mirror(&self) -> Option<usize> {
        (1..self.width).find(|&mirror| self.is_vertical_mirror(mirror))
    }

    fn find_horizontal_mirror(&self) -> Option<usize> {
        (1..self.height).find(|&mirror| self.is_horizontal_mirror(mirror))
    }
}

#[aoc(day13, part1)]
fn part1(input: &[Pattern]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, pattern)| {
            if let Some(mirror) = pattern.find_vertical_mirror() {
                mirror
            } else if let Some(mirror) = pattern.find_horizontal_mirror() {
                mirror * 100
            } else {
                panic!("no mirror found in pattern #{i}");
            }
        })
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Pattern]) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 405);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 0);
    }
}
