use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<String>;

#[aoc_generator(day1)]
fn parse(input: &str) -> Input {
    input.lines().map(String::from).collect()
}

#[aoc(day1, part1)]
fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|line| {
            let first_digit = line
                .chars()
                .find(|c| c.is_ascii_digit())
                .and_then(|c| c.to_digit(10))
                .unwrap();
            let last_digit = line
                .chars()
                .rfind(|c| c.is_ascii_digit())
                .and_then(|c| c.to_digit(10))
                .unwrap();
            first_digit * 10 + last_digit
        })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &Input) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part1(&parse(example)), 142);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
