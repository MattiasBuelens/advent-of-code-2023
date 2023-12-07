use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Write};
use std::hash::Hash;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Card(char);

impl Card {
    fn parse(c: char) -> Card {
        Card(c)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.0)
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Hand([Card; 5]);

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.chars().take(5).map(Card::parse).collect::<Vec<_>>();
        let cards = cards.try_into().unwrap();
        Ok(Self(cards))
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for card in &self.0 {
            Display::fmt(card, f)?
        }
        Ok(())
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[derive(Debug, Clone)]
struct Bid {
    hand: Hand,
    amount: u32,
}

impl FromStr for Bid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, amount) = s.split_once(' ').unwrap();
        let hand = hand.parse().unwrap();
        let amount = amount.parse().unwrap();
        Ok(Self { hand, amount })
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Bid> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

impl Card {
    fn power(self) -> u8 {
        match self.0 {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => 9,
            'T' => 8,
            c @ '2'..='9' => (c.to_digit(10).unwrap() as u8) - 2,
            c => panic!("invalid card {c}"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let cards = &self.0;
        let mut card_counts = HashMap::<Card, usize>::new();
        for card in cards {
            *card_counts.entry(*card).or_default() += 1;
        }
        let (&max_card, &max_count) = card_counts
            .iter()
            .max_by_key(|(_card, count)| **count)
            .unwrap();
        if max_count == 5 {
            return HandType::FiveOfAKind;
        }
        if max_count == 4 {
            return HandType::FourOfAKind;
        }
        let (_second_max_card, &second_max_count) = card_counts
            .iter()
            .filter(|(card, _count)| **card != max_card)
            .max_by_key(|(_card, count)| **count)
            .unwrap();
        if max_count == 3 && second_max_count == 2 {
            return HandType::FullHouse;
        }
        if max_count == 3 {
            return HandType::ThreeOfAKind;
        }
        if max_count == 2 && second_max_count == 2 {
            return HandType::TwoPair;
        }
        if max_count == 2 {
            return HandType::OnePair;
        }
        HandType::HighCard
    }

    fn power(&self) -> impl Iterator<Item = u8> + '_ {
        self.0.iter().map(|card| card.power())
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then_with(|| self.power().cmp(other.power()))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(day7, part1)]
fn part1(input: &[Bid]) -> u32 {
    let mut ranked_bids = input.to_vec();
    ranked_bids.sort_by_key(|bid| bid.hand);
    ranked_bids
        .into_iter()
        .enumerate()
        .map(|(rank, bid)| (rank as u32 + 1) * bid.amount)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Bid]) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 6440);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 0);
    }
}
