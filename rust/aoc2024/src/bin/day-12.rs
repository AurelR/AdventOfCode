use std::collections::{BTreeMap, BTreeSet};

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

fn part2(input: &str) -> String {
    let map = parse(input);
    let xmax = map.keys().map(|p| p.0).max().unwrap();
    let ymax = map.keys().map(|p| p.1).max().unwrap();

    let mut visited = BTreeSet::<Pos>::new();
    let mut result = 0;
    for (&pos, &plant) in &map {
        if visited.contains(&pos) {
            continue;
        }
        let mut region_stack = Vec::<Pos>::new();
        let mut region = Set::new();
        let mut area = 0;
        region_stack.push(pos);
        while let Some(pos) = region_stack.pop() {
            if visited.contains(&pos) {
                continue;
            }
            region.insert(pos);
            let n = neighbors(&map, pos, plant);
            area += 1;
            visited.insert(pos);
            region_stack.extend(n.into_iter().filter(|p| !visited.contains(p)));
        }

        let mut sides = 0;
        for x in -1..=xmax + 1 {
            let mut left_last = false;
            let mut inside_last = false;
            for y in -1..=ymax + 1 {
                let left = region.contains(&(x - 1, y));
                let inside = region.contains(&(x, y));
                match (left_last, inside_last, left, inside) {
                    (false, false, false, true) => sides += 1,
                    (true, true, false, true) => sides += 1,
                    (false, false, true, false) => sides += 1,
                    (true, true, true, false) => sides += 1,
                    (true, false, false, true) => sides += 1,
                    (false, true, true, false) => sides += 1,
                    _ => {}
                }
                left_last = left;
                inside_last = inside;
            }
        }

        for y in -1..=ymax + 1 {
            let mut top_last = false;
            let mut inside_last = false;
            for x in -1..=xmax + 1 {
                let top = region.contains(&(x, y - 1));
                let inside = region.contains(&(x, y));
                match (top_last, inside_last, top, inside) {
                    (false, false, false, true) => sides += 1,
                    (true, true, false, true) => sides += 1,
                    (false, false, true, false) => sides += 1,
                    (true, true, true, false) => sides += 1,
                    (true, false, false, true) => sides += 1,
                    (false, true, true, false) => sides += 1,
                    _ => {}
                }
                top_last = top;
                inside_last = inside;
            }
        }

        result += area * sides;
    }
    result.to_string()
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
    fn test_part2_a() {
        let input = "AAAA
BBCD
BBCC
EEEC
";

        let result = part2(input);
        assert_eq!("80", result);
    }

    #[test]
    fn test_part2_b() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

        let result = part2(input);
        assert_eq!("236", result);
    }

    #[test]
    fn test_part2_c() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

        let result = part2(input);
        assert_eq!("368", result);
    }

    #[test]
    fn test_part2_full() {
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
