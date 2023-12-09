use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::array_windows;

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| line.split(' ').map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn predict(sequence: &[i64]) -> i64 {
    if sequence.iter().all(|&x| x == 0) {
        return 0;
    }
    let differences = array_windows(sequence)
        .map(|[a, b]| b - a)
        .collect::<Vec<_>>();
    return sequence.last().unwrap() + predict(&differences);
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|sequence| predict(sequence)).sum()
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<i64>]) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 0);
    }
}
