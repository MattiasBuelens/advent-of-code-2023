use std::collections::{HashMap, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Module {
    name: String,
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
                name: name.to_string(),
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

impl Module {
    fn receive(&mut self, pulse: bool, sender: &str) -> Option<bool> {
        match &mut self.kind {
            ModuleKind::FlipFlop(state) => {
                // If a flip-flop module receives a low pulse, it flips between on and off.
                // If it was off, it turns on and sends a high pulse.
                // If it was on, it turns off and sends a low pulse.
                if !pulse {
                    *state = !*state;
                    Some(*state)
                } else {
                    None
                }
            }
            ModuleKind::Conjunction(state) => {
                // When a pulse is received, the conjunction module first updates its memory for that input.
                let sender_idx = self
                    .inputs
                    .iter()
                    .position(|input| input == sender)
                    .unwrap();
                state[sender_idx] = pulse;
                // Then, if it remembers high pulses for all inputs, it sends a low pulse;
                // otherwise, it sends a high pulse.
                if state.iter().all(|&x| x) {
                    Some(false)
                } else {
                    Some(true)
                }
            }
            ModuleKind::Broadcast => {
                // When it receives a pulse, it sends the same pulse to all of its destination modules.
                Some(pulse)
            }
        }
    }
}

#[derive(Debug)]
struct Pulse {
    value: bool,
    sender: String,
    receiver: String,
}

#[derive(Debug, Clone, Default)]
struct PulseCounts {
    low: u64,
    high: u64,
}

fn push_button(graph: &mut ModuleGraph, counts: &mut PulseCounts) {
    let mut queue = VecDeque::<Pulse>::new();
    // When you push the button, a single low pulse is sent directly to the broadcaster module.
    queue.push_back(Pulse {
        value: false,
        sender: "button".to_string(),
        receiver: "broadcaster".to_string(),
    });
    counts.low += 1;
    while let Some(pulse) = queue.pop_front() {
        if let Some(receiver) = graph.get_mut(&pulse.receiver) {
            if let Some(new_pulse) = receiver.receive(pulse.value, &pulse.sender) {
                for output in &receiver.outputs {
                    queue.push_back(Pulse {
                        value: new_pulse,
                        sender: receiver.name.clone(),
                        receiver: output.clone(),
                    });
                    if new_pulse {
                        counts.high += 1;
                    } else {
                        counts.low += 1;
                    }
                }
            }
        }
    }
}

#[aoc(day20, part1)]
fn part1(graph: &ModuleGraph) -> u64 {
    let mut graph = graph.clone();
    let mut counts = PulseCounts::default();
    for _ in 0..1000 {
        push_button(&mut graph, &mut counts);
    }
    counts.low * counts.high
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
