use std::collections::HashMap;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::Vector2D;

#[derive(Debug)]
struct Schematic {
    things: HashMap<Vector2D, EngineThing>,
}

#[derive(Debug)]
struct EngineThing {
    symbol: char,
    part_numbers: Vec<u32>,
}

impl FromStr for Schematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = Vec::new();
        let mut things = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            let mut current_number: Option<(Vector2D, String)> = None;
            for (x, c) in line.chars().enumerate() {
                let pos = Vector2D::new(x as i32, y as i32);
                if c.is_ascii_digit() {
                    let (_, number) = current_number.get_or_insert((pos, String::new()));
                    number.push(c);
                } else {
                    if c != '.' {
                        let thing = EngineThing {
                            symbol: c,
                            part_numbers: vec![],
                        };
                        things.insert(pos, thing);
                    }
                    if let Some((pos, number)) = current_number.take() {
                        numbers.push((pos, number));
                    }
                }
            }
            if let Some((pos, number)) = current_number.take() {
                numbers.push((pos, number));
            }
        }
        let mut result = Self { things };
        for (pos, number) in numbers {
            result.add_part_number(pos, &number);
        }
        Ok(result)
    }
}

impl Schematic {
    fn adjacent_symbol_pos(&mut self, pos: Vector2D, width: i32) -> Option<Vector2D> {
        for y in pos.y() - 1..=pos.y() + 1 {
            let min_x = pos.x() - 1;
            let max_x = pos.x() + width;
            for x in min_x..=max_x {
                let pos = Vector2D::new(x, y);
                if self.things.contains_key(&pos) {
                    return Some(pos);
                }
            }
        }
        None
    }

    fn add_part_number(&mut self, pos: Vector2D, number: &str) {
        if let Some(thing_pos) = self.adjacent_symbol_pos(pos, number.len() as i32) {
            self.things
                .get_mut(&thing_pos)
                .unwrap()
                .part_numbers
                .push(number.parse().unwrap())
        }
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Schematic {
    input.parse().unwrap()
}

#[aoc(day3, part1)]
fn part1(input: &Schematic) -> u32 {
    input
        .things
        .values()
        .flat_map(|thing| thing.part_numbers.iter())
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &Schematic) -> u32 {
    input
        .things
        .values()
        .filter(|thing| thing.symbol == '*' && thing.part_numbers.len() == 2)
        .map(|thing| thing.part_numbers[0] * thing.part_numbers[1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 4361);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 467835);
    }
}
