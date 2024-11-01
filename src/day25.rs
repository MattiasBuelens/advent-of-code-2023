use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use rand::seq::IteratorRandom;
use rand::SeedableRng;

#[derive(Debug, Default, Clone)]
struct Graph {
    connections: HashMap<String, Vec<String>>,
    weights: HashMap<String, usize>,
}

impl Graph {
    fn add(&mut self, from: &str, to: &str) {
        self.connections
            .entry(from.to_string())
            .or_default()
            .push(to.to_string());
        self.connections
            .entry(to.to_string())
            .or_default()
            .push(from.to_string());
        self.weights.entry(from.to_string()).or_insert(1);
        self.weights.entry(to.to_string()).or_insert(1);
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
                .collect::<Vec<_>>() // for size_hint()
        })
    }

    /// Remove the edge `(from, to)` from the graph, and replace it with a new node
    /// that is connected with all nodes connected to `from` or `to` in the original graph.
    /// https://en.wikipedia.org/wiki/Karger%27s_algorithm#Contraction_algorithm
    fn contract(&mut self, from: &str, to: &str) {
        // Merge `from` connections with `to` connections.
        let to_connections = self.connections.remove(to).unwrap();
        let from_connections = self.connections.get_mut(from).unwrap();
        from_connections.extend(to_connections);
        from_connections.retain(|x| x != from && x != to); // no self-loops

        // Merge weights.
        *self.weights.get_mut(from).unwrap() += self.weights.remove(to).unwrap();

        // Replace `to` with `from` in the rest of the graph.
        for (_, connections) in self.connections.iter_mut() {
            for other in connections.iter_mut() {
                if other == to {
                    *other = from.to_string();
                }
            }
        }
    }
}

const SEED: u64 = 2;

/// https://en.wikipedia.org/wiki/Karger's_algorithm
fn split_graph(input: &Graph) -> (usize, usize) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(SEED);
    let mut attempts = 0;
    loop {
        let mut graph = input.clone();
        while graph.connections.len() > 2 {
            let (from, to) = graph.edges().choose(&mut rng).unwrap();
            let (from, to) = (from.to_string(), to.to_string());
            graph.contract(&from, &to);
        }
        // Only two nodes left in the contracted graph
        // Check if they're separated by exactly 3 edges
        assert_eq!(graph.connections.len(), 2);
        let connections = graph.connections.values().next().unwrap();
        if connections.len() == 3 {
            // Found it!
            let weights = graph.weights.values().cloned().collect::<Vec<_>>();
            assert_eq!(weights.len(), 2);
            println!("Found after {attempts} attempts with seed {SEED}");
            return (weights[0], weights[1]);
        } else {
            // Accidentally merged the wrong two nodes. Try again.
            attempts += 1;
        }
    }
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
