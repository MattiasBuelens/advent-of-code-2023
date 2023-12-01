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

fn part2_parse_digit(s: &str) -> Option<u32> {
    if let Some(digit @ '1'..='9') = s.chars().next() {
        Some(digit.to_digit(10).unwrap())
    } else if s.starts_with("one") {
        Some(1)
    } else if s.starts_with("two") {
        Some(2)
    } else if s.starts_with("three") {
        Some(3)
    } else if s.starts_with("four") {
        Some(4)
    } else if s.starts_with("five") {
        Some(5)
    } else if s.starts_with("six") {
        Some(6)
    } else if s.starts_with("seven") {
        Some(7)
    } else if s.starts_with("eight") {
        Some(8)
    } else if s.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

#[aoc(day1, part2)]
fn part2(input: &Input) -> u32 {
    input
        .iter()
        .map(|line| {
            let first_digit = (0..line.len())
                .find_map(|i| part2_parse_digit(&line[i..]))
                .unwrap();
            let last_digit = (0..line.len())
                .rev()
                .find_map(|i| part2_parse_digit(&line[i..]))
                .unwrap();
            first_digit * 10 + last_digit
        })
        .sum()
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
        let example = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part2(&parse(example)), 281);
    }
}
