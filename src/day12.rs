use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Spring {
    Unknown,
    Operational,
    Damaged,
}

#[derive(Debug, Clone)]
struct Record {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '?' => Spring::Unknown,
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            c => panic!("invalid spring {c}"),
        }
    }
}

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, groups) = s.split_once(' ').unwrap();
        let springs = springs.chars().map(Spring::from).collect();
        let groups = groups.split(',').map(|x| x.parse().unwrap()).collect();
        Ok(Self { springs, groups })
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<Record> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn match_group(springs: &[Spring], group: usize) -> Option<&[Spring]> {
    if springs.len() < group {
        return None;
    }
    let (head, tail) = springs.split_at(group);
    // The first `group` elements must be damaged or unknown.
    if head.iter().all(|&spring| spring != Spring::Operational) {
        // Each group must be followed by a non-damaged spring.
        if let Some((&first_tail, tail)) = tail.split_first() {
            if first_tail != Spring::Damaged {
                Some(tail)
            } else {
                None
            }
        } else {
            Some(&[])
        }
    } else {
        None
    }
}

fn solve(springs: &[Spring], groups: &[usize]) -> usize {
    if springs.is_empty() {
        return if groups.is_empty() {
            1
        } else {
            // At least one unsatisfied group
            0
        };
    }
    if groups.is_empty() {
        return if springs.iter().any(|&spring| spring == Spring::Damaged) {
            // At least one damaged spring is not in a group
            0
        } else {
            1
        };
    }
    // Try to match the next group
    let (&first_group, remaining_groups) = groups.split_first().unwrap();
    let mut matches = 0;
    for (index, &spring) in springs.iter().enumerate() {
        match spring {
            Spring::Operational => continue,
            Spring::Damaged => {
                // A damaged spring *must* match in the next group.
                if let Some(remaining_springs) = match_group(&springs[index..], first_group) {
                    matches += solve(remaining_springs, remaining_groups);
                }
                return matches;
            }
            Spring::Unknown => {
                // An unknown spring may be damaged (matching the next group),
                // or may be operational (not yet matching).
                if let Some(remaining_springs) = match_group(&springs[index..], first_group) {
                    matches += solve(remaining_springs, remaining_groups);
                }
            }
        }
    }
    matches
}

#[aoc(day12, part1)]
fn part1(input: &[Record]) -> usize {
    input
        .iter()
        .map(|record| solve(&record.springs, &record.groups))
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &[Record]) -> usize {
    let new_input = input
        .iter()
        .map(|record| {
            let mut springs = vec![];
            let mut groups = vec![];
            for i in 0..5 {
                if i != 0 {
                    springs.push(Spring::Unknown);
                }
                springs.extend(record.springs.clone());
                groups.extend(record.groups.clone());
            }
            Record { springs, groups }
        })
        .collect::<Vec<_>>();
    part1(&new_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 525152);
    }
}
