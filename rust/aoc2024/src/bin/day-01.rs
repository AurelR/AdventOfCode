use std::collections::BTreeMap;

type NumTy = i32;
type Map = BTreeMap<NumTy, NumTy>;

fn main() {
    let input = std::fs::read_to_string("data/input/input01.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let mut d1 = Vec::new();
    let mut d2 = Vec::new();
    for l in input.lines() {
        let line = l.split_ascii_whitespace().collect::<Vec<_>>();
        let n1 = line[0].parse::<NumTy>().unwrap();
        let n2 = line[1].parse::<NumTy>().unwrap();
        d1.push(n1);
        d2.push(n2);
    }

    d1.sort();
    d2.sort();

    d1.into_iter()
        .zip(d2)
        .map(|(n1, n2)| n1.abs_diff(n2))
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    let mut d1 = Vec::new();
    let mut d2 = Map::new();
    for l in input.lines() {
        let line = l.split_ascii_whitespace().collect::<Vec<_>>();
        let n1 = line[0].parse::<NumTy>().unwrap();
        let n2 = line[1].parse::<NumTy>().unwrap();
        d1.push(n1);
        *d2.entry(n2).or_default() += 1;
    }

    d1.into_iter()
        .map(|n| d2.get(&n).unwrap_or(&0) * n)
        .sum::<NumTy>()
        .to_string()
}
