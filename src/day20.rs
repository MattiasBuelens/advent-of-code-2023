use std::collections::HashMap;

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
    time: usize,
}

fn push_button(graph: &mut ModuleGraph) -> Vec<Pulse> {
    let mut pulses = Vec::<Pulse>::new();
    // When you push the button, a single low pulse is sent directly to the broadcaster module.
    pulses.push(Pulse {
        value: false,
        sender: "button".to_string(),
        receiver: "broadcaster".to_string(),
        time: 0,
    });
    let mut index = 0;
    while index < pulses.len() {
        let pulse = &pulses[index];
        if let Some(receiver) = graph.get_mut(&pulse.receiver) {
            if let Some(new_pulse) = receiver.receive(pulse.value, &pulse.sender) {
                let new_pulses = receiver
                    .outputs
                    .iter()
                    .map(|output| Pulse {
                        value: new_pulse,
                        sender: receiver.name.clone(),
                        receiver: output.clone(),
                        time: pulse.time + 1,
                    })
                    .collect::<Vec<_>>();
                pulses.extend(new_pulses);
            }
        }
        index += 1;
    }
    pulses
}

#[aoc(day20, part1)]
fn part1(graph: &ModuleGraph) -> u64 {
    let mut graph = graph.clone();
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        let pulses = push_button(&mut graph);
        for pulse in pulses {
            if pulse.value {
                high_pulses += 1;
            } else {
                low_pulses += 1
            }
        }
    }
    low_pulses * high_pulses
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
