use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Scratchcard {
    winning: Vec<u32>,
    have: Vec<u32>,
}

impl FromStr for Scratchcard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_id, numbers) = s.split_once(": ").unwrap();
        let (winning, have) = numbers.split_once(" | ").unwrap();
        let winning = winning.split(' ').filter_map(|x| x.parse().ok()).collect();
        let have = have.split(' ').filter_map(|x| x.parse().ok()).collect();
        Ok(Self { winning, have })
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Scratchcard> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

impl Scratchcard {
    fn count_matches(&self) -> usize {
        self.winning
            .iter()
            .filter(|&&winning| self.have.contains(&winning))
            .count()
    }

    fn points(&self) -> u32 {
        match self.count_matches() as u32 {
            0 => 0,
            matches => 2u32.pow(matches - 1),
        }
    }
}

#[aoc(day4, part1)]
fn part1(input: &[Scratchcard]) -> u32 {
    input.iter().map(Scratchcard::points).sum()
}

#[aoc(day4, part2)]
fn part2(input: &[Scratchcard]) -> u32 {
    // Start with one original for each card
    let mut card_counts = vec![1u32; input.len()];
    for (card_idx, card) in input.iter().enumerate() {
        let card_count = card_counts[card_idx];
        let card_matches = card.count_matches();
        for i in 1..=card_matches {
            card_counts[card_idx + i] += card_count;
        }
    }
    card_counts.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 30);
    }
}
