use std::collections::BTreeMap;
use std::collections::BTreeSet;

type NumTy = i32;
type Pos = (NumTy, NumTy);
type Set = BTreeSet<Pos>;
type Map = BTreeMap<Pos, NumTy>;

fn main() {
    let input = std::fs::read_to_string("data/input/input10.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (_max, map) = parse(input);

    let mut result = 0;
    for (&start, &height) in map.iter() {
        if height != 0 {
            continue;
        }
        let mut targets = Set::new();
        count_trails(&map, start, height, &mut targets);
        result += targets.len();
    }
    result.to_string()
}

fn part2(input: &str) -> String {
    let (_max, map) = parse(input);

    let mut result = 0;
    for (&start, &height) in map.iter() {
        if height != 0 {
            continue;
        }
        result += count_trails2(&map, start, height);
    }
    result.to_string()
}

fn parse(input: &str) -> (Pos, Map) {
    let mut x_max = 0;
    let mut y_max = 0;
    let mut heights = Map::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            heights.insert((x as NumTy, y as NumTy), (c - 48) as NumTy);
            x_max = x_max.max(x as NumTy);
        }
        y_max = y_max.max(y as NumTy);
    }
    ((x_max, y_max), heights)
}

fn count_trails(map: &Map, pos: Pos, height: NumTy, targets: &mut Set) {
    if height == 9 {
        targets.insert(pos);
        return;
    }
    for (n_pos, n_height) in neighbors(map, pos, height) {
        count_trails(map, n_pos, n_height, targets);
    }
}

fn count_trails2(map: &Map, pos: Pos, height: NumTy) -> NumTy {
    if height == 9 {
        return 1;
    }
    let mut ret = 0;
    for (n_pos, n_height) in neighbors(map, pos, height) {
        ret += count_trails2(map, n_pos, n_height);
    }
    ret
}

fn neighbors(map: &Map, pos: (i32, i32), height: i32) -> Vec<(Pos, NumTy)> {
    let mut ret = Vec::new();
    if let Some((&n_pos, &n_h)) = map.get_key_value(&(pos.0 + 1, pos.1)) {
        if height + 1 == n_h {
            ret.push((n_pos, n_h));
        }
    }
    if let Some((&n_pos, &n_h)) = map.get_key_value(&(pos.0 - 1, pos.1)) {
        if height + 1 == n_h {
            ret.push((n_pos, n_h));
        }
    }
    if let Some((&n_pos, &n_h)) = map.get_key_value(&(pos.0, pos.1 + 1)) {
        if height + 1 == n_h {
            ret.push((n_pos, n_h));
        }
    }
    if let Some((&n_pos, &n_h)) = map.get_key_value(&(pos.0, pos.1 - 1)) {
        if height + 1 == n_h {
            ret.push((n_pos, n_h));
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

        let result = part1(input);
        assert_eq!("36", result);
    }

    #[test]
    fn test_part2() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

        let result = part2(input);
        assert_eq!("81", result);
    }
}
