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
    fn expand(&self) -> Self {
        let old_galaxies = &self.galaxies;
        let mut galaxies = Vec::new();
        let mut x_offset = 0;
        for x in 0..self.width {
            let column = old_galaxies
                .iter()
                .filter(|pos| pos.x() == x)
                .copied()
                .collect::<Vec<_>>();
            if column.is_empty() {
                x_offset += 1;
            } else {
                for mut galaxy in column {
                    *galaxy.x_mut() += x_offset;
                    galaxies.push(galaxy);
                }
            }
        }
        let old_galaxies = &galaxies;
        let mut galaxies = Vec::new();
        let mut y_offset = 0;
        for y in 0..self.height {
            let row = old_galaxies
                .iter()
                .filter(|pos| pos.y() == y)
                .copied()
                .collect::<Vec<_>>();
            if row.is_empty() {
                y_offset += 1;
            } else {
                for mut galaxy in row {
                    *galaxy.y_mut() += y_offset;
                    galaxies.push(galaxy);
                }
            }
        }
        let width = self.width + x_offset;
        let height = self.height + y_offset;
        Self {
            width,
            height,
            galaxies,
        }
    }
}

#[aoc(day11, part1)]
fn part1(input: &Image) -> i32 {
    let image = input.expand();
    let mut total_distance = 0;
    for (i, &left) in image.galaxies.iter().enumerate() {
        for &right in image.galaxies.iter().skip(i + 1) {
            let distance = (left - right).manhattan_distance();
            total_distance += distance;
        }
    }
    total_distance
}

#[aoc(day11, part2)]
fn part2(input: &Image) -> i32 {
    todo!()
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
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 0);
    }
}
