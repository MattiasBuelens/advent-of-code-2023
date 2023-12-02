use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Debug, Clone)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, sets) = s.split_once(": ").unwrap();
        let id = game.strip_prefix("Game ").unwrap().parse().unwrap();
        let sets = sets.split("; ").map(|set| set.parse().unwrap()).collect();
        Ok(Game { id, sets })
    }
}

impl FromStr for Set {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for cubes in s.split(", ") {
            let (count, color) = cubes.split_once(' ').unwrap();
            let count = count.parse().unwrap();
            match color {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => panic!("unknown color {color}"),
            }
        }
        Ok(Set { red, green, blue })
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Game> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Game]) -> u32 {
    let bag = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    let possible_games = input.iter().filter(|game| {
        game.sets
            .iter()
            .all(|set| set.red <= bag.red && set.green <= bag.green && set.blue <= bag.blue)
    });
    possible_games.map(|game| game.id).sum()
}

#[aoc(day2, part2)]
fn part2(_input: &[Game]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(&parse(input)), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
