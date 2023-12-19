use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::{array_windows, Direction, Vector2D};

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
fn part1(plan: &[Instruction]) -> u64 {
    let mut edge = Vec::<Vector2D>::new();
    // Dig out the edge
    let mut pos = Vector2D::new(0, 0);
    edge.push(pos);
    for instruction in plan {
        pos += instruction.dir.step() * instruction.meters;
        edge.push(pos);
    }
    // Dig out the interior
    trench_area(&edge)
}

fn trench_area(edge: &[Vector2D]) -> u64 {
    // https://en.wikipedia.org/wiki/Shoelace_formula#Trapezoid_formula
    assert_eq!(edge.first(), edge.last());
    let mut area = 0i64;
    let mut perimeter = 0i64;
    for &[pos, next_pos] in array_windows(edge) {
        area += (pos.y() + next_pos.y()) as i64 * (pos.x() - next_pos.x()) as i64;
        perimeter += (next_pos - pos).manhattan_distance() as i64;
    }
    area = area.abs() / 2;
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    // A = I + B/2 - 1
    // area = A and perimeter = B, but we actually need I + B
    // I = A - B/2 + 1
    // I + B = A + B/2 + 1
    // https://www.reddit.com/r/adventofcode/comments/18lg2we/comment/kdxd7yg/?utm_source=share&utm_medium=web2x&context=3
    area += perimeter / 2 + 1;
    area.abs() as u64
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
fn part2(plan: &[Instruction]) -> u64 {
    let plan = plan.iter().map(fix_instruction).collect::<Vec<_>>();
    part1(&plan)
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
        assert_eq!(part2(&parse(INPUT)), 952_408_144_115);
    }
}
