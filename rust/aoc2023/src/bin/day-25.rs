use petgraph::visit::Dfs;
use petgraph::{Graph, Undirected};
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = std::fs::read_to_string("data/input/input25.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
}

fn part1(input: &str) -> String {
    let graph_data = input
        .lines()
        .map(|l| {
            let (key, values) = l.split_once(": ").unwrap();
            (key, values.split(" ").collect::<BTreeSet<_>>())
        })
        .collect::<BTreeMap<_, _>>();

    // found it manually with graphviz and layout sfdp
    // use petgraph::dot::{Config, Dot};
    // println!("{:?}", Dot::with_config(&mst, &[Config::EdgeNoLabel]));
    let ignore = ["ffj", "xjb", "xhg", "lkm", "vgs", "ljl"];
    let mut graph = Graph::<&str, (), Undirected>::default();
    let mut node_to_index = BTreeMap::new();
    let mut edge_index = Vec::new();
    for (&n0, nodes) in graph_data.iter() {
        let idx0 = if !node_to_index.contains_key(n0) {
            let idx = graph.add_node(n0);
            node_to_index.insert(n0, idx);
            idx
        } else {
            *node_to_index.get(&n0).unwrap()
        };

        for &n1 in nodes.iter() {
            let idx1 = if !node_to_index.contains_key(n1) {
                let idx = graph.add_node(n1);
                node_to_index.insert(n1, idx);
                idx
            } else {
                *node_to_index.get(&n1).unwrap()
            };
            if !(ignore.contains(&n0) && ignore.contains(&n1)) {
                edge_index.push(graph.add_edge(idx0, idx1, ()));
            }
        }
    }

    let mut dfs = Dfs::new(&graph, node_to_index["ffj"]);
    let mut nodes = 0;
    while let Some(_) = dfs.next(&graph) {
        nodes += 1;
    }
    ((graph.node_count() - nodes) * nodes).to_string()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    #[ignore = "solution only works for real input"]
    fn test_part1() {
        let input = "\
jqt: rhn xhk nvd
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
frs: qnr lhk lsr
";

        let result = part1(input);
        assert_eq!("54", result);
    }
}
