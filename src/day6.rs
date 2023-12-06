use aoc_runner_derive::{aoc, aoc_generator};

struct Race {
    time: u64,
    distance: u64,
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Race> {
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
fn part1(input: &[Race]) -> u64 {
    input.iter().map(|race| race.ways_to_win()).product()
}

#[aoc(day6, part2)]
fn part2(input: &[Race]) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 288);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 0);
    }
}
