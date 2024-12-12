use std::collections::{BTreeMap, BTreeSet};

//use itertools::Itertools;

type NumTy = i32;
type Pos = (NumTy, NumTy);
type Map = BTreeMap<Pos, char>;
type Set = BTreeSet<Pos>;

fn main() {
    let input = std::fs::read_to_string("data/input/input12.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let map = parse(input);
    let mut visited = Set::new();
    let mut result = 0;
    for (&pos, &plant) in &map {
        if visited.contains(&pos) {
            continue;
        }
        let mut region = Vec::<Pos>::new();
        let mut area = 0;
        let mut perimeter = 0;
        region.push(pos);
        while let Some(pos) = region.pop() {
            if visited.contains(&pos) {
                continue;
            }
            let n = neighbors(&map, pos, plant);
            area += 1;
            perimeter += 4 - (n.len() as NumTy);
            visited.insert(pos);
            region.extend(n.into_iter().filter(|p| !visited.contains(p)));
        }
        result += area * perimeter;
    }
    result.to_string()
}

fn part2(_input: &str) -> String {
    "".to_string()
}

fn parse(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as NumTy, y as NumTy), c))
        })
        .collect()
}

fn neighbors(map: &Map, pos: (i32, i32), plant: char) -> Vec<Pos> {
    let mut ret = Vec::new();
    if let Some((&n_pos, &p)) = map.get_key_value(&(pos.0 + 1, pos.1)) {
        if plant == p {
            ret.push(n_pos);
        }
    }
    if let Some((&n_pos, &p)) = map.get_key_value(&(pos.0 - 1, pos.1)) {
        if plant == p {
            ret.push(n_pos);
        }
    }
    if let Some((&n_pos, &p)) = map.get_key_value(&(pos.0, pos.1 + 1)) {
        if plant == p {
            ret.push(n_pos);
        }
    }
    if let Some((&n_pos, &p)) = map.get_key_value(&(pos.0, pos.1 - 1)) {
        if plant == p {
            ret.push(n_pos);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

        let result = part1(input);
        assert_eq!("1930", result);
    }

    #[test]
    fn test_part2() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

        let result = part2(input);
        assert_eq!("1206", result);
    }
}
