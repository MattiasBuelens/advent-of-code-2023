use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::lcm_64;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

struct Node {
    left: String,
    right: String,
}

struct Input {
    steps: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Input {
    let (steps, nodes) = input.split_once("\n\n").unwrap();
    let steps = steps
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            c => panic!("invalid direction {c}"),
        })
        .collect();
    let nodes = nodes
        .lines()
        .map(|line| {
            let (label, next) = line.split_once(" = ").unwrap();
            let next = next.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
            let (left, right) = next.split_once(", ").unwrap();
            let node = Node {
                left: left.to_string(),
                right: right.to_string(),
            };
            (label.to_string(), node)
        })
        .collect();
    Input { steps, nodes }
}

fn solve(start: &String, is_goal: impl Fn(&String) -> bool, input: &Input) -> i64 {
    let mut current = start;
    let mut steps = input.steps.iter().cycle();
    let mut step_count = 0;
    while !is_goal(current) {
        let current_node = input.nodes.get(current).unwrap();
        let step = steps.next().unwrap();
        current = match step {
            Direction::Left => &current_node.left,
            Direction::Right => &current_node.right,
        };
        step_count += 1
    }
    step_count
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> i64 {
    solve(&"AAA".to_string(), |label| label == "ZZZ", input)
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> i64 {
    let start_labels = input
        .nodes
        .keys()
        .filter(|label| label.ends_with('A'))
        .collect::<Vec<_>>();
    let solutions = start_labels
        .iter()
        .map(|start| solve(start, |label| label.ends_with('Z'), input))
        .collect::<Vec<_>>();
    solutions
        .iter()
        .fold(1, |steps, &solution| lcm_64(steps, solution))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(&parse(input)), 6);
    }

    #[test]
    fn part2_example() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(part2(&parse(input)), 6);
    }
}
