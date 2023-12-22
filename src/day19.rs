use std::collections::HashMap;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Op {
    Greater,
    Less,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Destination {
    Workflow(String),
    Accept,
    Reject,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RatingId {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

#[derive(Debug, Clone)]
struct Rule {
    rating: RatingId,
    op: Op,
    value: i64,
    dest: Destination,
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
    default: Destination,
}

#[derive(Debug, Clone)]
struct Part {
    ratings: [i64; 4],
}

#[derive(Debug, Clone)]
struct Input {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl From<char> for RatingId {
    fn from(value: char) -> Self {
        match value {
            'x' => RatingId::X,
            'm' => RatingId::M,
            'a' => RatingId::A,
            's' => RatingId::S,
            _ => panic!("invalid rating ID: {value}"),
        }
    }
}

impl FromStr for Destination {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Destination::Accept,
            "R" => Destination::Reject,
            worfklow => Destination::Workflow(worfklow.to_string()),
        })
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rating, s) = s.split_at(1);
        let (op, s) = s.split_at(1);
        let (value, dest) = s.split_once(':').unwrap();
        let rating = rating.chars().next().unwrap().into();
        let op = match op {
            ">" => Op::Greater,
            "<" => Op::Less,
            _ => panic!("invalid op {op}"),
        };
        let value = value.parse().unwrap();
        let dest = dest.parse().unwrap();
        Ok(Rule {
            rating,
            op,
            value,
            dest,
        })
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules = s.split(',').collect::<Vec<_>>();
        let (default, rules) = rules.split_last().unwrap();
        let rules = rules.iter().map(|s| s.parse().unwrap()).collect();
        let default = default.parse().unwrap();
        Ok(Workflow { rules, default })
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('{').unwrap();
        let s = s.strip_suffix('}').unwrap();
        let ratings = s
            .split(',')
            .map(|s| {
                let (rating, value) = s.split_once('=').unwrap();
                assert_eq!(rating.len(), 1);
                let _rating: RatingId = rating.chars().next().unwrap().into();

                value.parse().unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Ok(Part { ratings })
    }
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Input {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|s| {
            let (name, s) = s.split_once('{').unwrap();
            let workflow = s.strip_suffix('}').unwrap().parse().unwrap();
            (name.to_string(), workflow)
        })
        .collect();
    let parts = parts.lines().map(|s| s.parse().unwrap()).collect();
    Input { workflows, parts }
}

impl Rule {
    fn matches(&self, part: &Part) -> bool {
        let rating = part.ratings[self.rating as usize];
        match self.op {
            Op::Greater => rating > self.value,
            Op::Less => rating < self.value,
        }
    }
}

fn process(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut workflow = workflows.get("in").unwrap();
    loop {
        let dest = workflow
            .rules
            .iter()
            .find_map(|rule| rule.matches(part).then_some(&rule.dest))
            .unwrap_or(&workflow.default);
        match dest {
            Destination::Accept => return true,
            Destination::Reject => return false,
            Destination::Workflow(name) => {
                workflow = workflows.get(name).unwrap();
            }
        }
    }
}

#[aoc(day19, part1)]
fn part1(input: &Input) -> i64 {
    input
        .parts
        .iter()
        .filter(|part| process(part, &input.workflows))
        .map(|part| part.ratings.iter().sum::<i64>())
        .sum()
}

#[derive(Debug, Clone)]
struct PartRange {
    ratings: [(i64, i64); 4],
}

impl PartRange {
    fn all() -> Self {
        Self {
            ratings: [(1, 4000); 4],
        }
    }

    fn is_empty(&self) -> bool {
        self.ratings.iter().any(|(start, end)| start > end)
    }

    /// Split into two halves.
    /// The left half contains everything up to (but not including) split_value.
    /// The right half contains everything starting from (and including) split_value.
    fn split(&self, split_id: RatingId, split_value: i64) -> [Self; 2] {
        let mut left_ratings = self.ratings;
        let mut right_ratings = self.ratings;
        let (_, left_end) = &mut left_ratings[split_id as usize];
        let (right_start, _) = &mut right_ratings[split_id as usize];
        *left_end = (*left_end).min(split_value - 1);
        *right_start = (*right_start).max(split_value);
        [
            Self {
                ratings: left_ratings,
            },
            Self {
                ratings: right_ratings,
            },
        ]
    }

    fn size(&self) -> i64 {
        if self.is_empty() {
            return 0;
        }
        self.ratings
            .iter()
            .map(|&(start, end)| end - start + 1)
            .product()
    }
}

fn process_range(
    range: PartRange,
    rules: &[Rule],
    default: &Destination,
    workflows: &HashMap<String, Workflow>,
) -> Vec<PartRange> {
    if range.is_empty() {
        return vec![];
    }
    if let Some((rule, rules)) = rules.split_first() {
        // Split into two halves: one that matches, and one that doesn't
        let (matching, failing) = match rule.op {
            Op::Greater => {
                let [less_or_equal, greater] = range.split(rule.rating, rule.value + 1);
                (greater, less_or_equal)
            }
            Op::Less => {
                let [less, greater_or_equal] = range.split(rule.rating, rule.value);
                (less, greater_or_equal)
            }
        };
        // The matching part goes to the rule's destination
        let matching_accepted = process_destination(matching, &rule.dest, workflows);
        // The failing part goes to the next rule
        let failing_accepted = process_range(failing, rules, default, workflows);
        // Combine accepted ranges from both paths
        let mut accepted = matching_accepted;
        accepted.extend(failing_accepted);
        accepted
    } else {
        // No more rules, follow default destination
        process_destination(range, default, workflows)
    }
}

fn process_destination(
    range: PartRange,
    dest: &Destination,
    workflows: &HashMap<String, Workflow>,
) -> Vec<PartRange> {
    match dest {
        Destination::Accept => vec![range],
        Destination::Reject => vec![],
        Destination::Workflow(name) => {
            let workflow = workflows.get(name).unwrap();
            process_range(range, &workflow.rules, &workflow.default, workflows)
        }
    }
}

#[aoc(day19, part2)]
fn part2(input: &Input) -> i64 {
    let in_rule = input.workflows.get("in").unwrap();
    let accepted = process_range(
        PartRange::all(),
        &in_rule.rules,
        &in_rule.default,
        &input.workflows,
    );
    accepted.iter().map(|range| range.size()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 19114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 167_409_079_868_000);
    }
}
