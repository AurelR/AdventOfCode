use std::collections::{BTreeMap, BTreeSet};

type Node<'a> = &'a str; //[u8; 2];
type Nodes<'a> = BTreeSet<Node<'a>>;
type Edges<'a> = BTreeSet<(Node<'a>, Node<'a>)>;
type AdjList<'a> = BTreeMap<Node<'a>, Nodes<'a>>;

fn main() {
    let input = std::fs::read_to_string("data/input/input23.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (nodes, _edges, adj_list) = parse(input);
    let mut three_cycles = BTreeSet::<Nodes>::new();
    for &n1 in &nodes {
        let n1n = &adj_list[n1];
        for &n2 in n1n {
            for &n3 in n1n.intersection(&adj_list[n2]) {
                if n3 == n1 {
                    continue;
                }
                let mut cycle = Nodes::new();
                cycle.insert(n1);
                cycle.insert(n2);
                cycle.insert(n3);
                three_cycles.insert(cycle);
            }
        }
    }

    three_cycles
        .into_iter()
        .filter(|c| c.iter().filter(|n| n.starts_with("t")).count() > 0)
        .count()
        .to_string()
}

fn part2(input: &str) -> String {
    let (_nodes, _edges, _adj_list) = parse(input);
    "".to_string()
}

fn parse(input: &str) -> (Nodes, Edges, AdjList) {
    let mut nodes = Nodes::new();
    let mut adj_list = AdjList::new();
    let edges = input
        .lines()
        .map(|l| {
            let n1: Node = &l[0..2];
            let n2: Node = &l[3..5];
            //let n1: Node = l.as_bytes()[0..2].try_into().unwrap();
            //let n2: Node = l.as_bytes()[3..5].try_into().unwrap();
            nodes.insert(n1);
            nodes.insert(n2);
            adj_list.entry(n1).or_default().insert(n2);
            adj_list.entry(n2).or_default().insert(n1);
            if n1 < n2 {
                (n1, n2)
            } else {
                (n2, n1)
            }
        })
        .collect();
    (nodes, edges, adj_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

        let result: String = part1(input);
        assert_eq!("7", result);
    }

    #[test]
    #[ignore = "todo"]
    fn test_part2() {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

        let result = part2(input);
        assert_eq!("co,de,ka,ta", result);
    }
}
