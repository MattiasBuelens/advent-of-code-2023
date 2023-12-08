use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use aoc_runner_derive::{aoc, aoc_generator};

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

#[aoc(day8, part1)]
fn part1(input: &Input) -> u32 {
    let mut current = &"AAA".to_string();
    let mut steps = input.steps.iter().cycle();
    let mut step_count = 0;
    while current != "ZZZ" {
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

#[derive(Debug, Clone)]
struct State<'a> {
    label: &'a String,
    step: usize,
    total_steps: usize,
}

impl<'a> PartialEq<Self> for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.label.eq(other.label) && self.step.eq(&other.step)
    }
}

impl<'a> Eq for State<'a> {}

impl<'a> Hash for State<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label.hash(state);
        self.step.hash(state);
    }
}

#[derive(Debug, Clone)]
struct Cycle {
    start: usize,
    length: usize,
    steps_from_start_to_destination: usize,
}

fn find_cycle(start: &String, input: &Input) -> Cycle {
    let mut steps = input.steps.iter().cycle();
    let mut states = HashSet::new();
    let mut state = State {
        label: start,
        step: 0,
        total_steps: 0,
    };
    let mut steps_to_destination = None;
    while !states.contains(&state) {
        states.insert(state.clone());
        let current_node = input.nodes.get(state.label).unwrap();
        let step = steps.next().unwrap();
        state.label = match step {
            Direction::Left => &current_node.left,
            Direction::Right => &current_node.right,
        };
        state.total_steps += 1;
        state.step = state.total_steps % input.steps.len();
        if state.label.ends_with('Z') && steps_to_destination.is_none() {
            steps_to_destination = Some(state.total_steps);
        }
    }
    let first_state = states.get(&state).unwrap();
    Cycle {
        start: first_state.total_steps,
        length: state.total_steps - first_state.total_steps,
        steps_from_start_to_destination: steps_to_destination.unwrap() - first_state.total_steps,
    }
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> u32 {
    let start_labels = input
        .nodes
        .keys()
        .filter(|label| label.ends_with('A'))
        .collect::<Vec<_>>();
    let cycles = start_labels
        .iter()
        .map(|start| (start, find_cycle(start, input)))
        .collect::<Vec<_>>();
    dbg!(&cycles);
    todo!()
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
