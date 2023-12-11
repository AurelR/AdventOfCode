type NumTy = i32;
type Position = (NumTy, NumTy);
use std::collections::BTreeSet;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("data/input/input11.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let data = parse_input(input);
    data.iter()
        .tuple_combinations()
        .map(|(p1, p2)| p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1))
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    let _data = parse_input(input);
    "".to_string()
}

#[allow(unused_imports)]
fn parse_input(input: &str) -> BTreeSet<Position> {
    let mut postions = Vec::new();
    let mut empty_cols: Vec<bool> = vec![true; input.lines().next().unwrap().len()];
    let mut y = 0;
    for line in input.lines() {
        let mut empty_row = true;
        for (x, c) in line.as_bytes().iter().enumerate() {
            match &c {
                b'#' => {
                    empty_row = false;
                    empty_cols[x] = false;
                    postions.push((x as NumTy, y));
                }
                b'.' => {}
                _ => panic!("Unexpected input"),
            }
        }
        if empty_row {
            y += 1;
        }
        y += 1;
    }

    for (i, c) in empty_cols
        .into_iter()
        .enumerate()
        .filter_map(|(col, empty)| if empty { Some(col) } else { None })
        .enumerate()
    {
        for pos in postions.iter_mut() {
            if pos.0 > (c + i) as NumTy {
                pos.0 += 1;
            }
        }
    }
    BTreeSet::from_iter(postions.into_iter())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_expansion() {
        let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let expected = "\
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
";
        let mut positions = BTreeSet::new();
        for (y, line) in expected.lines().enumerate() {
            for (x, c) in line.as_bytes().iter().enumerate() {
                match &c {
                    b'#' => {
                        positions.insert((x as NumTy, y as NumTy));
                    }
                    b'.' => {}
                    _ => panic!("Unexpected input"),
                }
            }
        }
        let result = parse_input(input);
        assert_eq!(result, positions);
    }

    #[test]
    fn test_part1() {
        let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let result = part1(input);
        assert_eq!(result, "374");
    }

    #[test]
    fn test_part2() {
        let input = "\
";
        let result = part2(input);
        assert_eq!(result, "todo");
    }
}
