use foldhash::HashMap;

use crate::{solution::Solution, util::invalid_input};

pub fn solution() -> Solution {
    Solution::new().with_a(a).with_b(b)
}

fn parse<'i>(input: &'i str) -> anyhow::Result<Graph<'i>> {
    let mut graph = Graph::default();
    for line in input.lines() {
        let (from, to) = line.split_once(": ").ok_or_else(invalid_input!())?;
        let from = graph.insert_node(from);
        for to in to.split_ascii_whitespace() {
            let to = graph.insert_node(to);
            graph.insert_edge(from, to);
        }
    }

    Ok(graph)
}

fn get_node_idx(network: &Graph, name: &str) -> anyhow::Result<NodeIdx> {
    let idx = *network
        .names
        .get(name)
        .ok_or_else(invalid_input!("missing '{name}' node"))?;
    Ok(idx)
}

fn a(input: &str) -> anyhow::Result<u64> {
    let network = parse(input)?;
    let you = get_node_idx(&network, "you")?;
    let out = get_node_idx(&network, "out")?;
    let count = count_paths(&network, you, out);
    Ok(count)
}

fn b(input: &str) -> anyhow::Result<u64> {
    let network = parse(input)?;
    let svr = get_node_idx(&network, "svr")?;
    let dac = get_node_idx(&network, "dac")?;
    let fft = get_node_idx(&network, "fft")?;
    let out = get_node_idx(&network, "out")?;

    let dac_fft = count_paths(&network, dac, fft);
    let fft_dac = count_paths(&network, fft, dac);

    let (int1, int2, int_paths) = match (dac_fft, fft_dac) {
        (0, 0) => return Ok(0),
        (n, 0) => (dac, fft, n),
        (0, m) => (fft, dac, m),
        (_, _) => return Err(anyhow::anyhow!("unexpected cyclic graph")),
    };

    let svr_int1 = count_paths(&network, svr, int1);
    let int2_out = count_paths(&network, int2, out);

    let count = svr_int1 * int_paths * int2_out;
    Ok(count)
}

fn count_paths(network: &Graph, from: NodeIdx, to: NodeIdx) -> u64 {
    fn inner(network: &Graph, from: NodeIdx, to: NodeIdx, cache: &mut [Option<u64>]) -> u64 {
        if from == to {
            return 1;
        }

        if let Some(cached) = cache[from.0 as usize] {
            return cached;
        }

        let from_node = network.node(from);
        let mut count = 0;
        for &next in from_node.neighbors.iter() {
            count += inner(network, next, to, cache);
        }

        cache[from.0 as usize] = Some(count);
        count
    }

    let mut cache = vec![None; network.nodes.len()];
    inner(network, from, to, &mut cache)
}

#[derive(Debug, Default, Clone)]
struct Graph<'n> {
    names: HashMap<&'n str, NodeIdx>,
    nodes: Vec<Node<'n>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct NodeIdx(u32);

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Node<'n> {
    name: &'n str,
    idx: NodeIdx,
    neighbors: Vec<NodeIdx>,
}

impl<'n> Graph<'n> {
    pub fn insert_node(&mut self, name: &'n str) -> NodeIdx {
        if let Some(&idx) = self.names.get(name) {
            idx
        } else {
            let idx = self.next_node_idx();
            let node = Node::new(name, idx);
            self.nodes.push(node);
            self.names.insert(name, idx);
            idx
        }
    }

    pub fn insert_edge(&mut self, from: NodeIdx, to: NodeIdx) {
        self.node_mut(from).neighbors.push(to);
    }

    fn next_node_idx(&self) -> NodeIdx {
        NodeIdx(self.nodes.len() as u32)
    }

    fn node(&self, idx: NodeIdx) -> &Node<'n> {
        &self.nodes[idx.0 as usize]
    }

    fn node_mut(&mut self, idx: NodeIdx) -> &mut Node<'n> {
        &mut self.nodes[idx.0 as usize]
    }
}

impl<'n> Node<'n> {
    pub fn new(name: &'n str, idx: NodeIdx) -> Self {
        Self {
            name,
            idx,
            neighbors: Vec::new(),
        }
    }
}

#[allow(dead_code)]
struct Dot<'g>(&'g Graph<'g>);

impl std::fmt::Display for Dot<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "strict digraph {{")?;

        for node in self.0.nodes.iter() {
            let attrs = match node.name {
                "you" | "out" | "svr" | "dac" | "fft" => {
                    ",style=filled,fillcolor=black,fontcolor=white"
                }
                _ => "",
            };

            writeln!(f, "n{} [label=\"{}\"{attrs}]", node.idx.0, node.name)?;
        }

        for node in self.0.nodes.iter() {
            for neighbor in node.neighbors.iter() {
                writeln!(f, "n{} -> n{}", node.idx.0, neighbor.0)?;
            }
        }

        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    const TEST_INPUT_A: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    #[rstest]
    #[case(TEST_INPUT_A, 5)]
    fn test_a(#[case] input: &str, #[case] expected: u64) {
        let result = super::a(input).unwrap();
        assert_eq!(result, expected);
    }

    const TEST_INPUT_B: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[rstest]
    #[case(TEST_INPUT_B, 2)]
    fn test_b(#[case] input: &str, #[case] expected: u64) {
        let result = super::b(input).unwrap();
        assert_eq!(result, expected);
    }
}
