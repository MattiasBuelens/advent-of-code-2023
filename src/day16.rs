use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::{Direction, Vector2D};

enum Piece {
    MirrorUpRight,
    MirrorDownRight,
    SplitterHorizontal,
    SplitterVertical,
}

struct Contraption {
    width: i32,
    height: i32,
    pieces: HashMap<Vector2D, Piece>,
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Contraption {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    let mut pieces = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let piece = match c {
                '.' => None,
                '/' => Some(Piece::MirrorUpRight),
                '\\' => Some(Piece::MirrorDownRight),
                '-' => Some(Piece::SplitterHorizontal),
                '|' => Some(Piece::SplitterVertical),
                _ => panic!("invalid piece {c}"),
            };
            if let Some(piece) = piece {
                pieces.insert(Vector2D::new(x as i32, y as i32), piece);
            }
        }
    }
    Contraption {
        width,
        height,
        pieces,
    }
}

impl Direction {
    fn mirror_upright(self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::N,
            Direction::W => Direction::S,
            Direction::S => Direction::W,
        }
    }

    fn mirror_downright(self) -> Direction {
        match self {
            Direction::N => Direction::W,
            Direction::W => Direction::N,
            Direction::S => Direction::E,
            Direction::E => Direction::S,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Beam {
    pos: Vector2D,
    dir: Direction,
}

impl Beam {
    fn new(pos: Vector2D, dir: Direction) -> Self {
        Self { pos, dir }
    }
}

type Beams = HashMap<Vector2D, HashSet<Direction>>;

impl Contraption {
    fn step(&self, beam: Beam) -> Vec<Beam> {
        let next_pos = beam.pos + beam.dir.step();
        match (self.pieces.get(&next_pos), beam.dir) {
            (None, _)
            | (Some(Piece::SplitterHorizontal), Direction::W | Direction::E)
            | (Some(Piece::SplitterVertical), Direction::N | Direction::S) => {
                // beam goes straight
                vec![Beam::new(next_pos, beam.dir)]
            }
            (Some(Piece::MirrorUpRight), dir) => {
                // beam is reflected
                vec![Beam::new(next_pos, dir.mirror_upright())]
            }
            (Some(Piece::MirrorDownRight), dir) => {
                // beam is reflected
                vec![Beam::new(next_pos, dir.mirror_downright())]
            }
            (Some(Piece::SplitterHorizontal), Direction::N | Direction::S) => {
                // beam is split
                vec![
                    Beam::new(next_pos, Direction::W),
                    Beam::new(next_pos, Direction::E),
                ]
            }
            (Some(Piece::SplitterVertical), Direction::W | Direction::E) => {
                // beam is split
                vec![
                    Beam::new(next_pos, Direction::N),
                    Beam::new(next_pos, Direction::S),
                ]
            }
        }
    }

    fn is_in_bounds(&self, pos: &Vector2D) -> bool {
        (0..self.width).contains(&pos.x()) && (0..self.height).contains(&pos.y())
    }
}

fn solve(input: &Contraption, start_beam: Beam) -> usize {
    let mut beams = Beams::new();
    let mut queue = VecDeque::new();
    queue.push_back(start_beam);
    while let Some(beam) = queue.pop_front() {
        for new_beam in input.step(beam) {
            if !input.is_in_bounds(&new_beam.pos) {
                continue;
            }
            if beams.entry(new_beam.pos).or_default().insert(new_beam.dir) {
                // new beam was inserted, push back onto queue
                queue.push_back(new_beam)
            }
        }
    }
    beams.len()
}

#[aoc(day16, part1)]
fn part1(input: &Contraption) -> usize {
    let start_beam = Beam::new(Vector2D::new(-1, 0), Direction::E);
    solve(input, start_beam)
}

#[aoc(day16, part2)]
fn part2(input: &Contraption) -> usize {
    let mut max_energized = 0;
    for x in 0..input.width {
        let from_top = Beam::new(Vector2D::new(x, -1), Direction::S);
        let from_bottom = Beam::new(Vector2D::new(x, input.height), Direction::N);
        max_energized = max_energized.max(solve(input, from_top));
        max_energized = max_energized.max(solve(input, from_bottom));
    }
    for y in 0..input.height {
        let from_left = Beam::new(Vector2D::new(-1, y), Direction::E);
        let from_right = Beam::new(Vector2D::new(input.width, y), Direction::W);
        max_energized = max_energized.max(solve(input, from_left));
        max_energized = max_energized.max(solve(input, from_right));
    }
    max_energized
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 46);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 51);
    }
}
