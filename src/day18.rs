use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Interval {
    from: i32,
    to: i32,
}

impl Interval {
    fn new(from: i32, to: i32) -> Self {
        debug_assert!(from <= to);
        Self { from, to }
    }

    fn contains(&self, value: i32) -> bool {
        self.from <= value && value <= self.to
    }
}

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

impl Rectangle {
    fn from_points(from: Vector2D, to: Vector2D) -> Self {
        debug_assert!(from.x() <= to.x());
        debug_assert!(from.y() <= to.y());
        Self {
            left: from.x(),
            top: from.y(),
            width: to.x() - from.x() + 1,
            height: to.y() - from.y() + 1,
        }
    }

    fn right(&self) -> i32 {
        self.left + self.width
    }

    fn bottom(&self) -> i32 {
        self.top + self.height
    }

    fn expand(self, other: Self) -> Self {
        let left = self.left.min(other.left);
        let top = self.top.min(other.top);
        let width = self.right().max(other.right()) - left;
        let height = self.bottom().max(other.bottom()) - top;
        Self {
            left,
            top,
            width,
            height,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct IntervalSet {
    list: Vec<Interval>,
}

impl IntervalSet {
    fn new() -> Self {
        Self { list: Vec::new() }
    }

    fn insert(&mut self, interval: Interval) {
        let index = self
            .list
            .binary_search(&interval)
            .expect_err("interval already exists");
        self.list.insert(index, interval);
    }

    fn find(&self, value: i32) -> Option<&Interval> {
        self.list
            .binary_search_by(|interval| {
                if interval.to < value {
                    Ordering::Less
                } else if interval.from > value {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
            .ok()
            .map(|index| &self.list[index])
    }

    fn bounds(&self) -> Option<Interval> {
        if self.list.is_empty() {
            return None;
        }
        return Some(Interval::new(
            self.list.first().unwrap().from,
            self.list.last().unwrap().to,
        ));
    }
}

#[derive(Debug, Clone)]
struct LineMap {
    horizontal: HashMap<i32, IntervalSet>,
    vertical: HashMap<i32, IntervalSet>,
}

impl LineMap {
    fn new() -> Self {
        Self {
            horizontal: HashMap::new(),
            vertical: HashMap::new(),
        }
    }

    fn insert(&mut self, left: Vector2D, right: Vector2D) {
        let (left, right) = (left.min(right), left.max(right));
        if left.y() == right.y() {
            self.horizontal
                .entry(left.y())
                .or_default()
                .insert(Interval::new(left.x(), right.x()));
        } else if left.x() == right.x() {
            self.vertical
                .entry(left.x())
                .or_default()
                .insert(Interval::new(left.y(), right.y()));
        } else {
            panic!("not a line: left = {left:?}, right = {right:?}");
        }
    }

    fn find(&self, pos: &Vector2D) -> Option<(Vector2D, Vector2D)> {
        self.horizontal
            .get(&pos.y())
            .and_then(|set| set.find(pos.x()))
            .map(|interval| {
                (
                    Vector2D::new(interval.from, pos.y()),
                    Vector2D::new(interval.to, pos.y()),
                )
            })
            .or_else(|| {
                self.vertical
                    .get(&pos.x())
                    .and_then(|set| set.find(pos.y()))
                    .map(|interval| {
                        (
                            Vector2D::new(pos.x(), interval.from),
                            Vector2D::new(pos.x(), interval.to),
                        )
                    })
            })
    }

    fn contains(&self, pos: &Vector2D) -> bool {
        self.find(pos).is_some()
    }

    fn bounds(&self) -> Option<Rectangle> {
        let mut bounds: Option<Rectangle> = None;
        for (&y, set) in &self.horizontal {
            if let Some(set_interval) = set.bounds() {
                let set_bounds = Rectangle::from_points(
                    Vector2D::new(set_interval.from, y),
                    Vector2D::new(set_interval.to, y),
                );
                bounds = Some(match bounds {
                    Some(bounds) => bounds.expand(set_bounds),
                    None => set_bounds,
                });
            }
        }
        for (&x, set) in &self.vertical {
            if let Some(set_interval) = set.bounds() {
                let set_bounds = Rectangle::from_points(
                    Vector2D::new(x, set_interval.from),
                    Vector2D::new(x, set_interval.to),
                );
                bounds = Some(match bounds {
                    Some(bounds) => bounds.expand(set_bounds),
                    None => set_bounds,
                });
            }
        }
        bounds
    }
}

#[aoc(day18, part1)]
fn part1(plan: &[Instruction]) -> usize {
    let mut edge = LineMap::new();
    // Dig out the edge
    let mut pos = Vector2D::new(0, 0);
    for instruction in plan {
        let new_pos = pos + (instruction.dir.step() * instruction.meters);
        edge.insert(pos, new_pos);
        pos = new_pos;
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

fn fill_trench(edge: &LineMap) -> HashSet<Vector2D> {
    let bounds = edge.bounds().unwrap();
    let min_x = bounds.left;
    let max_x = bounds.right();
    let min_y = bounds.top;
    let max_y = bounds.bottom();
    let mut trench = HashSet::new();
    for y in min_y..=max_y {
        let mut following = FollowingState::NotFollowing;
        let mut inside = false;
        for x in min_x..=max_x {
            let pos = Vector2D::new(x, y);
            let on_edge = edge.contains(&pos);
            let pos_up = pos + Vector2D::new(0, -1);
            let pos_down = pos + Vector2D::new(0, 1);
            if following == FollowingState::FollowingUp {
                // Following an edge with a turn at the top
                if !on_edge {
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
                if !on_edge {
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
            } else if on_edge {
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
            if inside || on_edge {
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
