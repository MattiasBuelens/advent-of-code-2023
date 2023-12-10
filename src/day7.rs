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
    fn power(self, part2: bool) -> u8 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' if part2 => 1,
            'J' => 11,
            'T' => 10,
            c @ '2'..='9' => c.to_digit(10).unwrap() as u8,
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
    fn hand_type(&self, part2: bool) -> HandType {
        let cards = &self.0;
        let mut card_counts = HashMap::<Card, usize>::new();
        for card in cards {
            *card_counts.entry(*card).or_default() += 1;
        }
        let joker_count = if part2 {
            card_counts.get(&Card('J')).copied().unwrap_or_default()
        } else {
            0
        };
        if joker_count == 5 {
            // All cards are jokers
            return HandType::FiveOfAKind;
        }
        let (&max_card, &max_count) = card_counts
            .iter()
            .filter(|(card, _count)| {
                // Skip jokers
                !(part2 && **card == Card('J'))
            })
            .max_by_key(|(_card, count)| **count)
            .unwrap();
        // Add jokers to card with highest count
        let max_count = max_count + joker_count;
        if max_count == 5 {
            return HandType::FiveOfAKind;
        }
        if max_count == 4 {
            return HandType::FourOfAKind;
        }
        let (_second_max_card, &second_max_count) = card_counts
            .iter()
            .filter(|(card, _count)| {
                // Skip jokers
                !(part2 && **card == Card('J'))
            })
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

    fn power(&self, part2: bool) -> impl Iterator<Item = u8> + '_ {
        self.0.iter().map(move |card| card.power(part2))
    }
}

fn compare_hands(left: &Hand, right: &Hand, part2: bool) -> Ordering {
    left.hand_type(part2)
        .cmp(&right.hand_type(part2))
        .then_with(|| left.power(part2).cmp(right.power(part2)))
}

fn solve(bids: &[Bid], part2: bool) -> u32 {
    let mut ranked_bids = bids.to_vec();
    ranked_bids.sort_by(|left, right| compare_hands(&left.hand, &right.hand, part2));
    ranked_bids
        .into_iter()
        .enumerate()
        .map(|(rank, bid)| (rank as u32 + 1) * bid.amount)
        .sum()
}

#[aoc(day7, part1)]
fn part1(input: &[Bid]) -> u32 {
    solve(input, false)
}

#[aoc(day7, part2)]
fn part2(input: &[Bid]) -> u32 {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
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
        assert_eq!(part2(&parse(INPUT)), 5905);
    }
}
