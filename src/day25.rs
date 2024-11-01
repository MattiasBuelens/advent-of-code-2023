use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::undirected::connected_components::connected_components;

#[derive(Debug, Default, Clone)]
struct Graph {
    connections: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn add(&mut self, from: &str, to: &str) {
        self.connections
            .entry(from.to_string())
            .or_default()
            .insert(to.to_string());
        self.connections
            .entry(to.to_string())
            .or_default()
            .insert(from.to_string());
    }
}

#[aoc_generator(day25)]
fn parse(input: &str) -> Graph {
    let mut graph = Graph::default();
    for line in input.lines() {
        let (from, connections) = line.split_once(": ").unwrap();
        for to in connections.split(' ') {
            graph.add(from, to);
        }
    }
    graph
}

#[aoc(day25, part1)]
fn part1(graph: &Graph) -> usize {
    let (left, right) = split_graph(graph);
    left * right
}

impl Graph {
    fn remove(&mut self, from: &str, to: &str) {
        self.connections.entry(from.to_string()).and_modify(|x| {
            x.remove(to);
        });
        self.connections.entry(to.to_string()).and_modify(|x| {
            x.remove(from);
        });
    }

    fn edges(&self) -> impl Iterator<Item = (&str, &str)> {
        self.connections.iter().flat_map(|(from, connections)| {
            connections
                .iter()
                .filter(move |to| {
                    // Each edge is stored twice in the map, but we only want to return them once
                    // Break symmetry by requiring alphabetical order
                    from < to
                })
                .map(move |to| (from.as_ref(), to.as_ref()))
        })
    }

    fn connected_components(&self) -> Vec<usize> {
        let starts = self.connections.keys().collect::<Vec<_>>();
        let components = connected_components(&starts[..], |&vertex| {
            self.connections.get(vertex).into_iter().flatten()
        });
        components
            .into_iter()
            .map(|component| component.len())
            .collect()
    }
}

fn combinations<T>(edges: &[T]) -> impl Iterator<Item = [&T; 3]> {
    edges
        .iter()
        .enumerate()
        .flat_map(move |(first_index, first)| {
            edges.iter().enumerate().skip(first_index + 1).flat_map(
                move |(second_index, second)| {
                    edges
                        .iter()
                        .skip(second_index + 1)
                        .map(move |third| [first, second, third])
                },
            )
        })
}

fn split_graph(graph: &Graph) -> (usize, usize) {
    let edges = graph.edges().collect::<Vec<_>>();
    for edges_to_remove in combinations(&edges) {
        let mut graph = graph.clone();
        for (from, to) in edges_to_remove {
            graph.remove(from, to);
        }
        match graph.connected_components()[..] {
            [first, second] => {
                return (first, second);
            }
            [_first] => {
                // Graph is still fully connected, keep going
            }
            ref result => {
                panic!("Graph was split into {} components?!", result.len())
            }
        }
    }
    panic!("Cannot split graph");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn part1_example() {
        let graph = parse(INPUT);
        let (left, right) = split_graph(&graph);
        assert_eq!(left * right, 54);
    }
}
