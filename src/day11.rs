use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::Vector2D;

#[derive(Debug, Clone)]
struct Image {
    width: i32,
    height: i32,
    galaxies: Vec<Vector2D>,
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Image {
    let width = input.lines().count() as i32;
    let height = input.lines().next().unwrap().len() as i32;
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Vector2D::new(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect();
    Image {
        width,
        height,
        galaxies,
    }
}

impl Image {
    fn empty_columns(&self) -> Vec<i32> {
        (0..self.width)
            .filter(|&x| self.galaxies.iter().filter(|pos| pos.x() == x).count() == 0)
            .collect()
    }

    fn empty_rows(&self) -> Vec<i32> {
        (0..self.height)
            .filter(|&y| self.galaxies.iter().filter(|pos| pos.y() == y).count() == 0)
            .collect()
    }

    fn expand(mut self, expansion: i32) -> Self {
        let empty_columns = self.empty_columns();
        let empty_rows = self.empty_rows();
        for galaxy in self.galaxies.iter_mut() {
            let x_offset = empty_columns.iter().filter(|&&x| x < galaxy.x()).count() as i32;
            let y_offset = empty_rows.iter().filter(|&&y| y < galaxy.y()).count() as i32;
            *galaxy += Vector2D::new(x_offset * expansion, y_offset * expansion);
        }
        self.width += empty_columns.len() as i32;
        self.height += empty_rows.len() as i32;
        self
    }

    fn total_distance(&self) -> i64 {
        let mut total_distance = 0;
        for (i, &left) in self.galaxies.iter().enumerate() {
            for &right in self.galaxies.iter().skip(i + 1) {
                let distance = (left - right).manhattan_distance();
                total_distance += distance as i64;
            }
        }
        total_distance
    }
}

#[aoc(day11, part1)]
fn part1(input: &Image) -> i64 {
    input.clone().expand(1).total_distance()
}

#[aoc(day11, part2)]
fn part2(input: &Image) -> i64 {
    input.clone().expand(1_000_000).total_distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 374);
    }

    #[test]
    fn part2_example1() {
        let input = parse(INPUT);
        assert_eq!(input.expand(10).total_distance(), 1030);
    }

    #[test]
    fn part2_example2() {
        let input = parse(INPUT);
        assert_eq!(input.expand(100).total_distance(), 8410);
    }
}
