use std::collections::BinaryHeap;

use foldhash::{HashMap, HashSet};
use itertools::Itertools;

use crate::{
    solution::Solution,
    util::{
        invalid_input,
        vector::{IVec3, vec3},
    },
};

pub fn solution() -> Solution {
    Solution::new().with_a(|i| a(i, 1000)).with_b(b)
}

fn parse_node(line: &str) -> nom::IResult<&str, IVec3> {
    use nom::{
        Parser,
        character::{char, complete::i64},
    };

    let mut parser = (i64, char(','), i64, char(','), i64).map(|(x, _, y, _, z)| vec3(x, y, z));
    parser.parse(line)
}

fn parse(input: &str) -> anyhow::Result<Vec<IVec3>> {
    let parsed: Vec<_> = input
        .lines()
        .map(|l| parse_node(l).map(|(_, n)| n))
        .collect::<Result<_, _>>()
        .map_err(invalid_input!(e))?;
    Ok(parsed)
}

fn a(input: &str, connections: u64) -> anyhow::Result<u64> {
    let nodes = parse(input)?;
    let mut candidates = Edge::candidates(&nodes);
    let mut circuits = Circuits::new(nodes.len());

    for _ in 0..connections {
        let edge = candidates
            .pop()
            .ok_or_else(|| anyhow::anyhow!("not enough edge candidates"))?;
        circuits.connect(edge.left, edge.right);
    }

    let mut circuit_sizes: Vec<_> = circuits
        .iter()
        .map(|circuit| circuit.len() as u64)
        .collect();
    circuit_sizes.sort_unstable();
    let a = circuit_sizes.pop().unwrap_or(1);
    let b = circuit_sizes.pop().unwrap_or(1);
    let c = circuit_sizes.pop().unwrap_or(1);

    Ok(a * b * c)
}

fn b(input: &str) -> anyhow::Result<i64> {
    let nodes = parse(input)?;
    let mut candidates = Edge::candidates(&nodes);
    let mut circuits = Circuits::new(nodes.len());

    let last_edge;
    loop {
        let edge = candidates
            .pop()
            .ok_or_else(|| anyhow::anyhow!("not enough edge candidates"))?;
        circuits.connect(edge.left, edge.right);
        if circuits.is_single_circuit() {
            last_edge = edge;
            break;
        }
    }

    let left = nodes[last_edge.left];
    let right = nodes[last_edge.right];
    Ok(left.x * right.x)
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    left: usize,
    right: usize,
    d2: i64,
}

impl Edge {
    pub fn candidates(nodes: &[IVec3]) -> BinaryHeap<Edge> {
        nodes
            .iter()
            .enumerate()
            .tuple_combinations()
            .map(|((li, l), (ri, r))| {
                let delta = r - l;

                Edge {
                    left: li,
                    right: ri,
                    d2: delta.dot(&delta),
                }
            })
            .collect()
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.d2 == other.d2
    }
}

impl Eq for Edge {}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.d2.cmp(&self.d2)
    }
}

#[derive(Debug, Clone)]
struct Circuits {
    circuits: HashMap<usize, HashSet<usize>>,
    index: Vec<usize>,
}

impl Circuits {
    pub fn new(size: usize) -> Self {
        let circuits = (0..size).map(|i| (i, HashSet::from_iter([i]))).collect();
        let index = (0..size).collect();

        Self { circuits, index }
    }

    pub fn is_single_circuit(&self) -> bool {
        self.len() == 1
    }

    pub fn len(&self) -> usize {
        self.circuits.len()
    }

    pub fn connect(&mut self, left: usize, right: usize) -> bool {
        let lci = self.get_node_circuit(left);
        let rci = self.get_node_circuit(right);

        if lci != rci {
            self.merge_circuits(lci, rci);
            true
        } else {
            false
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &HashSet<usize>> {
        self.circuits.values()
    }

    fn get_node_circuit(&self, node: usize) -> usize {
        self.index[node]
    }

    fn merge_circuits(&mut self, dst: usize, src: usize) {
        let rc = self
            .circuits
            .remove(&src)
            .expect("source circuit should exist");
        let lc = self
            .circuits
            .get_mut(&dst)
            .expect("destination circuit should exist");

        for &node in rc.iter() {
            self.index[node] = dst;
        }

        lc.extend(rc);
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[rstest]
    #[case(TEST_INPUT, 10, 40)]
    fn test_a(#[case] input: &str, #[case] connections: u64, #[case] expected: u64) {
        let result = super::a(input, connections).unwrap();
        assert_eq!(result, expected);
    }
    #[rstest]
    #[case(TEST_INPUT, 25272)]
    fn test_b(#[case] input: &str, #[case] expected: i64) {
        let result = super::b(input).unwrap();
        assert_eq!(result, expected);
    }
}
