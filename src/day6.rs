use aoc_runner_derive::aoc;

struct Race {
    time: u64,
    distance: u64,
}

fn parse_part1(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines.next().unwrap().strip_prefix("Time:").unwrap();
    let distances = lines.next().unwrap().strip_prefix("Distance:").unwrap();
    let times = times
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse().unwrap());
    let distances = distances
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse().unwrap());
    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

impl Race {
    fn simulate(&self, hold_time: u64) -> u64 {
        let speed = hold_time;
        let move_time = self.time.saturating_sub(hold_time);
        speed * move_time
    }

    fn ways_to_win(&self) -> u64 {
        let mut ways = 0;
        for hold_time in 1..self.time {
            if self.simulate(hold_time) > self.distance {
                ways += 1;
            }
        }
        ways
    }
}

#[aoc(day6, part1)]
fn part1(input: &str) -> u64 {
    let races = parse_part1(input);
    races.iter().map(|race| race.ways_to_win()).product()
}

fn parse_part2(input: &str) -> Race {
    let mut lines = input.lines();
    let time = lines.next().unwrap().strip_prefix("Time:").unwrap();
    let distance = lines.next().unwrap().strip_prefix("Distance:").unwrap();
    let time = time
        .chars()
        .filter(|&c| c != ' ')
        .collect::<String>()
        .parse()
        .unwrap();
    let distance = distance
        .chars()
        .filter(|&c| c != ' ')
        .collect::<String>()
        .parse()
        .unwrap();
    Race { time, distance }
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u64 {
    let race = parse_part2(input);
    race.ways_to_win()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 288);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 71503);
    }
}
