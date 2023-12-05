use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u32>,
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
    dest_start: u32,
    source_start: u32,
    range_length: u32,
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
    fn map(&self, value: u32) -> u32 {
        for entry in &self.entries {
            if (entry.source_start..entry.source_start + entry.range_length).contains(&value) {
                return value - entry.source_start + entry.dest_start;
            }
        }
        value
    }
}

impl Almanac {
    fn seed_to_loc(&self, seed: u32) -> u32 {
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
fn part1(input: &Almanac) -> u32 {
    input
        .seeds
        .iter()
        .map(|seed| input.seed_to_loc(*seed))
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &Almanac) -> String {
    todo!()
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
        assert_eq!(part2(&parse(INPUT)), "<RESULT>");
    }
}
