use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Module {
    kind: ModuleKind,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

#[derive(Debug, Clone)]
enum ModuleKind {
    FlipFlop(bool),
    Conjunction(Vec<bool>),
    Broadcast,
}

type ModuleGraph = HashMap<String, Module>;

#[aoc_generator(day20)]
fn parse(input: &str) -> ModuleGraph {
    let mut graph = ModuleGraph::new();
    for line in input.lines() {
        let (name, outputs) = line.split_once(" -> ").unwrap();
        let (kind, name) = if name == "broadcaster" {
            (ModuleKind::Broadcast, name)
        } else if let Some(name) = name.strip_prefix('%') {
            (ModuleKind::FlipFlop(false), name)
        } else if let Some(name) = name.strip_prefix('&') {
            (ModuleKind::Conjunction(vec![]), name)
        } else {
            panic!("invalid module: {name}")
        };
        let outputs = outputs.split(", ").map(|s| s.to_string()).collect();
        graph.insert(
            name.to_string(),
            Module {
                kind,
                inputs: vec![],
                outputs,
            },
        );
    }
    // Copy outputs to inputs
    let module_names = graph.keys().cloned().collect::<Vec<_>>();
    for name in module_names {
        let outputs = graph.get(&name).unwrap().outputs.clone();
        for output in outputs {
            if let Some(other_module) = graph.get_mut(&output) {
                other_module.inputs.push(name.clone());
                // Add extra entry in conjunction module's state
                if let ModuleKind::Conjunction(state) = &mut other_module.kind {
                    state.push(false);
                }
            }
        }
    }
    graph
}

#[aoc(day20, part1)]
fn part1(graph: &ModuleGraph) -> u64 {
    dbg!(&graph);
    todo!()
}

#[aoc(day20, part2)]
fn part2(graph: &ModuleGraph) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    const EXAMPLE2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse(EXAMPLE1)), 32_000_000);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse(EXAMPLE2)), 11_687_500);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE1)), 0);
    }
}
