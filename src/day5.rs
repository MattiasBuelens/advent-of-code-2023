use std::cmp::Ordering;
use std::ops::Range;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<i64>,
    seed_to_soil: Mapping,
    soil_to_fert: Mapping,
    fert_to_water: Mapping,
    water_to_light: Mapping,
    light_to_temp: Mapping,
    temp_to_humid: Mapping,
    humid_to_loc: Mapping,
}

#[derive(Debug, Clone)]
struct Mapping {
    entries: Vec<MappingEntry>,
}

#[derive(Debug, Clone)]
struct MappingEntry {
    dest_start: i64,
    source_start: i64,
    range_length: i64,
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split("\n\n");
        let seeds = sections.next().unwrap().strip_prefix("seeds: ").unwrap();
        let seeds = seeds.split(' ').map(|x| x.parse().unwrap()).collect();
        let seed_to_soil = sections.next().unwrap().parse().unwrap();
        let soil_to_fert = sections.next().unwrap().parse().unwrap();
        let fert_to_water = sections.next().unwrap().parse().unwrap();
        let water_to_light = sections.next().unwrap().parse().unwrap();
        let light_to_temp = sections.next().unwrap().parse().unwrap();
        let temp_to_humid = sections.next().unwrap().parse().unwrap();
        let humid_to_loc = sections.next().unwrap().parse().unwrap();
        Ok(Self {
            seeds,
            seed_to_soil,
            soil_to_fert,
            fert_to_water,
            water_to_light,
            light_to_temp,
            temp_to_humid,
            humid_to_loc,
        })
    }
}

impl FromStr for Mapping {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().skip(1);
        let entries = lines.map(|line| line.parse().unwrap()).collect();
        Ok(Self { entries })
    }
}

impl FromStr for MappingEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.split(' ').map(|x| x.parse().unwrap());
        let dest_start = numbers.next().unwrap();
        let source_start = numbers.next().unwrap();
        let range_length = numbers.next().unwrap();
        Ok(Self {
            dest_start,
            source_start,
            range_length,
        })
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Almanac {
    input.parse().unwrap()
}

impl Mapping {
    fn map(&self, value: i64) -> i64 {
        for entry in &self.entries {
            if (entry.source_start..entry.source_start + entry.range_length).contains(&value) {
                return value - entry.source_start + entry.dest_start;
            }
        }
        value
    }
}

impl Almanac {
    fn seed_to_loc(&self, seed: i64) -> i64 {
        let soil = self.seed_to_soil.map(seed);
        let fert = self.soil_to_fert.map(soil);
        let water = self.fert_to_water.map(fert);
        let light = self.water_to_light.map(water);
        let temp = self.light_to_temp.map(light);
        let humid = self.temp_to_humid.map(temp);

        self.humid_to_loc.map(humid)
    }
}

#[aoc(day5, part1)]
fn part1(input: &Almanac) -> i64 {
    input
        .seeds
        .iter()
        .map(|seed| input.seed_to_loc(*seed))
        .min()
        .unwrap()
}

type Interval = Range<i64>;

fn compare_interval(a: &Interval, b: &Interval) -> Ordering {
    a.start.cmp(&b.start).then_with(|| a.end.cmp(&b.end))
}

#[derive(Debug, Clone)]
struct IntervalSet {
    intervals: Vec<Interval>,
}

impl IntervalSet {
    fn from(intervals: Vec<Interval>) -> Self {
        let mut set = Self { intervals };
        set.merge_overlapping();
        set
    }

    fn add_all(&mut self, new_ranges: &[Interval]) {
        self.intervals.extend_from_slice(new_ranges);
        self.merge_overlapping();
    }

    fn remove_and_shift(&mut self, shift_range: &Interval, offset: i64) -> Vec<Interval> {
        let mut new_intervals = Vec::<Interval>::with_capacity(self.intervals.capacity());
        let mut shifted_intervals = vec![];
        for range in &self.intervals {
            if shift_range.start <= range.start && range.end <= shift_range.end {
                // shift entire range
                shifted_intervals.push((range.start + offset)..(range.end + offset));
            } else if shift_range.end <= range.start || range.end <= shift_range.start {
                // keep range
                new_intervals.push(range.clone());
            } else if range.start < shift_range.start && shift_range.end < range.end {
                // shift middle of range
                new_intervals.push(range.start..shift_range.start);
                shifted_intervals.push((shift_range.start + offset)..(shift_range.end + offset));
                new_intervals.push(shift_range.end..range.end);
            } else if range.start < shift_range.start {
                // shift end of range
                new_intervals.push(range.start..shift_range.start);
                shifted_intervals.push((shift_range.start + offset)..(range.end + offset));
            } else {
                // shift start of range
                assert!(shift_range.end < range.end);
                shifted_intervals.push((range.start + offset)..(shift_range.end + offset));
                new_intervals.push(shift_range.end..range.end);
            }
        }
        self.intervals = new_intervals;
        self.merge_overlapping();
        shifted_intervals
    }

    fn merge_overlapping(&mut self) {
        self.intervals.sort_by(compare_interval);
        let mut i = 0;
        while i + 1 < self.intervals.len() {
            let prev_range = &self.intervals[i];
            let next_range = &self.intervals[i + 1];
            if prev_range.start <= next_range.end && next_range.start <= prev_range.end {
                let start = prev_range.start.min(next_range.start);
                let end = prev_range.end.max(next_range.end);
                self.intervals[i] = start..end;
                self.intervals.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }
}

impl Mapping {
    fn map_ranges(&self, ranges: IntervalSet) -> IntervalSet {
        let mut remainder = ranges.clone();
        let mut shifted = vec![];
        for entry in &self.entries {
            let range = entry.source_start..entry.source_start + entry.range_length;
            let offset = entry.dest_start - entry.source_start;
            shifted.extend_from_slice(&remainder.remove_and_shift(&range, offset));
        }
        remainder.add_all(&shifted);
        remainder
    }
}

impl Almanac {
    fn seed_to_loc_ranges(&self, seed: IntervalSet) -> IntervalSet {
        let soil = self.seed_to_soil.map_ranges(seed);
        let fert = self.soil_to_fert.map_ranges(soil);
        let water = self.fert_to_water.map_ranges(fert);
        let light = self.water_to_light.map_ranges(water);
        let temp = self.light_to_temp.map_ranges(light);
        let humid = self.temp_to_humid.map_ranges(temp);
        self.humid_to_loc.map_ranges(humid)
    }
}

#[aoc(day5, part2)]
fn part2(input: &Almanac) -> i64 {
    let mut seed = vec![];
    for start_and_length in input.seeds.chunks_exact(2) {
        if let [start, length] = start_and_length {
            seed.push(*start..(start + length));
        }
    }
    let seed = IntervalSet::from(seed);
    let loc = input.seed_to_loc_ranges(seed);
    loc.intervals.first().unwrap().start
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 46);
    }
}
