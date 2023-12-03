use std::collections::HashMap;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::Vector2D;

#[derive(Debug)]
struct Schematic {
    numbers: HashMap<Vector2D, String>,
    symbols: HashMap<Vector2D, char>,
}

impl FromStr for Schematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = HashMap::new();
        let mut symbols = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            let mut current_number: Option<(Vector2D, String)> = None;
            for (x, c) in line.chars().enumerate() {
                let pos = Vector2D::new(x as i32, y as i32);
                if c.is_ascii_digit() {
                    let (_, number) = current_number.get_or_insert((pos, String::new()));
                    number.push(c);
                } else {
                    if c != '.' {
                        symbols.insert(pos, c);
                    }
                    if let Some((pos, number)) = current_number.take() {
                        numbers.insert(pos, number);
                    }
                }
            }
            if let Some((pos, number)) = current_number.take() {
                numbers.insert(pos, number);
            }
        }
        Ok(Self { numbers, symbols })
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Schematic {
    input.parse().unwrap()
}

impl Schematic {
    fn part_numbers(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .filter(|(pos, number)| {
                let width = number.len() as i32;
                
                (pos.y() - 1..=pos.y() + 1).any(move |y| {
                    let min_x = pos.x() - 1;
                    let max_x = pos.x() + width;
                    (min_x..=max_x).any(move |x| self.symbols.contains_key(&Vector2D::new(x, y)))
                })
            })
            .map(|(_pos, number)| number.parse().unwrap())
            .collect()
    }
}

#[aoc(day3, part1)]
fn part1(input: &Schematic) -> u32 {
    input.part_numbers().into_iter().sum()
}

#[aoc(day3, part2)]
fn part2(_input: &Schematic) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part1(&parse(input)), 4361);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
