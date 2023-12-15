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

#[aoc(day15, part2)]
fn part2(input: &[String]) -> u32 {
    todo!()
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
        assert_eq!(part2(&parse(INPUT)), 0);
    }
}
