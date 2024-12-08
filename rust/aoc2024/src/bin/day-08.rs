use itertools::Itertools;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

type NumTy = i32;
type Pos = (NumTy, NumTy);
//type Set = BTreeSet<Pos>;
//type Map = BTreeMap<Pos, Set>;
type Map = BTreeMap<Pos, char>;

fn main() {
    let input = std::fs::read_to_string("data/input/input08.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (max, base_map) = parse(input);
    let rx = 0..= max.0;
    let ry = 0..=max.1;
    let map = invert(&base_map);
    let mut result = BTreeSet::<Pos>::new();
    for (_, p) in map.into_iter() {
        for c in p.into_iter().combinations(2) {
            let dx = c[0].0 - c[1].0;
            let dy = c[0].1 - c[1].1;
            let r1 = (c[0].0 + dx, c[0].1 +dy);
            let r2 = (c[1].0 - dx, c[1].1 -dy);
            if rx.contains(&r1.0) && ry.contains(&r1.1){
                result.insert(r1);
            }
            
            
            if rx.contains(&r2.0) && ry.contains(&r2.1){
                result.insert(r2);
            }
        }
    }
    //dbg!(&result);
    result.len().to_string()
}

fn part2(input: &str) -> String {
    // let (max, base_map) = parse(input);
    // let map = invert(&base_map);
    // let mut result = 0;
    // result.to_string()
    "".to_string()
}

fn invert(base_map: &BTreeMap<(i32, i32), char>) -> BTreeMap<char, Vec<Pos>> {
    let mut result = BTreeMap::<char, Vec<(i32, i32)>>::new();
    for (&p, &c) in base_map {
        result.entry(c).or_default().push(p);
    }
    result
}

fn parse(input: &str) -> (Pos, Map) {
    let mut x_max = 0;
    let mut y_max = 0;
    let mut antennas = Map::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.insert((x as NumTy, y as NumTy), c);
            }
            x_max = x_max.max(x as NumTy);
        }
        y_max = y_max.max(y as NumTy);
    }
    ((x_max, y_max), antennas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

        let result = part1(input);
        assert_eq!("14", result);
    }

    #[test]
    fn test_part2() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

        let result = part2(input);
        assert_eq!("000", result);
    }
}
