use std::array;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
fn parse(input: &str) -> Vec<String> {
    input.split(',').map(|s| s.to_string()).collect()
}

fn hash(s: &str) -> u8 {
    let mut value = 0u8;
    for byte in s.bytes() {
        value = ((value as u32 + byte as u32) * 17) as u8;
    }
    value
}

#[aoc(day15, part1)]
fn part1(input: &[String]) -> u32 {
    input.iter().map(|s| hash(s) as u32).sum()
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u32,
}

#[derive(Debug)]
struct HashMap {
    boxes: [Vec<Lens>; 256],
}

impl HashMap {
    fn new() -> Self {
        Self {
            boxes: array::from_fn(|_| Vec::new()),
        }
    }

    fn remove(&mut self, label: &str) {
        let lens_box = &mut self.boxes[hash(label) as usize];
        if let Some(slot_index) = lens_box.iter().position(|lens| lens.label == label) {
            lens_box.remove(slot_index);
        }
    }

    fn insert(&mut self, new_lens: Lens) {
        let lens_box = &mut self.boxes[hash(&new_lens.label) as usize];
        if let Some(slot_index) = lens_box
            .iter()
            .position(|lens| lens.label == new_lens.label)
        {
            lens_box[slot_index] = new_lens;
        } else {
            lens_box.push(new_lens);
        }
    }

    fn power(&self) -> u32 {
        self.boxes
            .iter()
            .enumerate()
            .map(move |(box_index, lens_box)| {
                lens_box
                    .iter()
                    .enumerate()
                    .map(move |(slot_index, lens)| {
                        (box_index as u32 + 1) * (slot_index as u32 + 1) * lens.focal_length
                    })
                    .sum::<u32>()
            })
            .sum()
    }
}

enum Operation {
    Remove(String),
    Insert(String, u32),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(label) = s.strip_suffix('-') {
            Ok(Operation::Remove(label.to_string()))
        } else if let Some((label, focal_length)) = s.split_once('=') {
            Ok(Operation::Insert(
                label.to_string(),
                focal_length.parse().unwrap(),
            ))
        } else {
            panic!("invalid operation: {s}")
        }
    }
}

#[aoc(day15, part2)]
fn part2(input: &[String]) -> u32 {
    let mut hashmap = HashMap::new();
    let operations = input
        .iter()
        .map(|s| Operation::from_str(s).unwrap())
        .collect::<Vec<_>>();
    for op in operations {
        match op {
            Operation::Remove(label) => hashmap.remove(&label),
            Operation::Insert(label, focal_length) => hashmap.insert(Lens {
                label,
                focal_length,
            }),
        };
    }
    hashmap.power()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 1320);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 145);
    }
}
