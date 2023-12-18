use std::collections::HashSet;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::{Direction, Vector2D};

#[derive(Debug, Clone)]
struct Instruction {
    dir: Direction,
    meters: i32,
    color: String,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, s) = s.split_once(' ').unwrap();
        let (meters, color) = s.split_once(' ').unwrap();
        let dir = match dir {
            "U" => Direction::N,
            "D" => Direction::S,
            "L" => Direction::W,
            "R" => Direction::E,
            _ => panic!("invalid direction {dir}"),
        };
        let meters = meters.parse().unwrap();
        let color = color
            .strip_prefix("(#")
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .to_string();
        Ok(Instruction { dir, meters, color })
    }
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day18, part1)]
fn part1(plan: &[Instruction]) -> usize {
    let mut edge = HashSet::<Vector2D>::new();
    // Dig out the edge
    let mut pos = Vector2D::new(0, 0);
    edge.insert(pos);
    for instruction in plan {
        for _ in 0..instruction.meters {
            pos += instruction.dir.step();
            edge.insert(pos);
        }
    }
    // Dig out the interior
    let trench = fill_trench(&edge);
    trench.len()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum FollowingState {
    NotFollowing,
    FollowingUp,
    FollowingDown,
}

fn fill_trench(edge: &HashSet<Vector2D>) -> HashSet<Vector2D> {
    let min_x = edge.iter().map(|pos| pos.x()).min().unwrap();
    let max_x = edge.iter().map(|pos| pos.x()).max().unwrap();
    let min_y = edge.iter().map(|pos| pos.y()).min().unwrap();
    let max_y = edge.iter().map(|pos| pos.y()).max().unwrap();
    let mut trench = edge.clone();
    for y in min_y..=max_y {
        let mut following = FollowingState::NotFollowing;
        let mut inside = false;
        for x in min_x..=max_x {
            let pos = Vector2D::new(x, y);
            let pos_up = pos + Vector2D::new(0, -1);
            let pos_down = pos + Vector2D::new(0, 1);
            if following == FollowingState::FollowingUp {
                // Following an edge with a turn at the top
                if !edge.contains(&pos) {
                    // Stopped following the edge
                    let prev_pos_up = pos + Vector2D::new(-1, -1);
                    if edge.contains(&prev_pos_up) {
                        // Turn at the same side, no crossing
                    } else {
                        // Turn on other side
                        inside = !inside
                    }
                    following = FollowingState::NotFollowing;
                }
            } else if following == FollowingState::FollowingDown {
                // Following an edge with a turn at the bottom
                if !edge.contains(&pos) {
                    // Stopped following the edge
                    let prev_pos_down = pos + Vector2D::new(-1, 1);
                    if edge.contains(&prev_pos_down) {
                        // Turn at the same side, no crossing
                    } else {
                        // Turn on other side
                        inside = !inside
                    }
                    following = FollowingState::NotFollowing;
                }
            } else if edge.contains(&pos) {
                match (edge.contains(&pos_up), edge.contains(&pos_down)) {
                    (true, true) => {
                        // Crossed the edge
                        inside = !inside;
                    }
                    (true, false) => {
                        // Start following an edge, turn is at the top
                        following = FollowingState::FollowingUp;
                    }
                    (false, true) => {
                        // Start following an edge, turn is at the bottom
                        following = FollowingState::FollowingDown;
                    }
                    (false, false) => panic!("invalid crossing at {pos:?}"),
                };
            }
            if inside {
                trench.insert(pos);
            }
        }
    }
    trench
}

fn fix_instruction(instruction: &Instruction) -> Instruction {
    let (meters_hex, dir_hex) = instruction.color.split_at(instruction.color.len() - 1);
    let meters = i32::from_str_radix(meters_hex, 16).unwrap();
    let dir = match u8::from_str_radix(dir_hex, 16).unwrap() {
        0 => Direction::E,
        1 => Direction::S,
        2 => Direction::W,
        3 => Direction::N,
        dir => panic!("invalid hex direction: {dir:x}"),
    };
    Instruction {
        dir,
        meters,
        color: instruction.color.clone(),
    }
}

#[aoc(day18, part2)]
fn part2(plan: &[Instruction]) -> usize {
    let plan = plan.iter().map(fix_instruction).collect::<Vec<_>>();
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 62);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 0);
    }
}
