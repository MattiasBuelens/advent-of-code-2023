use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
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

    fn solve(&self, prev_solution: Option<usize>) -> Option<usize> {
        for mirror in 1..self.width {
            if self.is_vertical_mirror(mirror) && prev_solution != Some(mirror) {
                return Some(mirror);
            }
        }
        for mirror in 1..self.height {
            if self.is_horizontal_mirror(mirror) && prev_solution != Some(mirror * 100) {
                return Some(mirror * 100);
            }
        }
        None
    }

    fn flip(mut self, index: usize) -> Self {
        let cell = &mut self.cells[index / self.width][index % self.width];
        *cell = match *cell {
            '.' => '#',
            '#' => '.',
            _ => panic!("invalid cell"),
        };
        self
    }
}

#[aoc(day13, part1)]
fn part1(input: &[Pattern]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, pattern)| {
            pattern.solve(None).unwrap_or_else(|| {
                panic!("no mirror found in pattern #{i}");
            })
        })
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Pattern]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, pattern)| {
            let prev_solution = pattern.solve(None).unwrap_or_else(|| {
                panic!("no mirror found in pattern #{i}");
            });
            for index in 0..(pattern.width * pattern.height) {
                let pattern = pattern.clone().flip(index);
                if let Some(new_solution) = pattern.solve(Some(prev_solution)) {
                    return new_solution;
                }
            }
            panic!("no smudge found in pattern #{i}");
        })
        .sum()
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
        assert_eq!(part2(&parse(INPUT)), 400);
    }
}
