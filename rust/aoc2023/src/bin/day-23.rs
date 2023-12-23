#![allow(unused_imports)]
type NumTy = i32;
use nom::character::complete::i32 as num_parser;
type Position = (NumTy, NumTy);
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    ops::RangeInclusive,
};

fn main() {
    let input = std::fs::read_to_string("data/input/input23.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (start, target, terrain) = parse_input(input);
    let mut path = BTreeSet::new();
    path.insert(start);
    let mut current_paths = Vec::new();
    current_paths.push((start, path));

    let mut final_paths = Vec::new();
    while let Some((mut current, mut path)) = current_paths.pop() {
        loop {
            let neigbors = get_neigbors(current)
                .into_iter()
                .filter(|n| {
                    let allowed = match terrain.get(n) {
                        Some(Tile::Path) => true,
                        Some(Tile::SlopeRight) => {
                            if n.0 == current.0 - 1 {
                                false
                            } else {
                                true
                            }
                        }
                        Some(Tile::SlopeLeft) => {
                            if n.0 == current.0 + 1 {
                                false
                            } else {
                                true
                            }
                        }
                        Some(Tile::SlopeUp) => {
                            if n.1 == current.1 + 1 {
                                false
                            } else {
                                true
                            }
                        }
                        Some(Tile::SlopeDown) => {
                            if n.1 == current.1 - 1 {
                                false
                            } else {
                                true
                            }
                        }
                        None => false,
                    };
                    allowed && !path.contains(n)
                })
                .collect::<Vec<_>>();
            if !neigbors.is_empty() {
                current = neigbors[0];
                for &n in &neigbors[1..] {
                    let mut new_path = path.clone();
                    new_path.insert(n);
                    current_paths.push((n, new_path));
                }
                path.insert(current);
                if current == target {
                    final_paths.push(path);
                    break;
                }
            } else {
                break;
            }
        }
    }
    final_paths
        .into_iter()
        .map(|p| p.len() - 1)
        .max()
        .unwrap()
        .to_string()
}

fn part2(input: &str) -> String {
    let _data = parse_input(input);
    "".to_string()
}

fn get_neigbors(pos: Position) -> [Position; 4] {
    let (x, y) = pos;
    [(x + 1, y), (x, y - 1), (x - 1, y), (x, y + 1)]
}

fn parse_input(input: &str) -> (Position, Position, BTreeMap<Position, Tile>) {
    let start = (1, 0);
    let mut target = (0, 0);
    let mut terrain = BTreeMap::new();
    for (y, line) in input.lines().enumerate() {
        let y = y as NumTy;
        for (x, c) in line.as_bytes().iter().enumerate() {
            let x = x as NumTy;
            match c {
                b'.' => {
                    terrain.insert((x, y), Tile::Path);
                }
                b'>' => {
                    terrain.insert((x, y), Tile::SlopeRight);
                }
                b'<' => {
                    terrain.insert((x, y), Tile::SlopeLeft);
                }
                b'^' => {
                    terrain.insert((x, y), Tile::SlopeUp);
                }
                b'v' => {
                    terrain.insert((x, y), Tile::SlopeDown);
                }
                b'#' => {} // Forest
                _ => unreachable!("expected tile character"),
            }
            target.0 = target.0.max(x);
        }
        target.1 = target.1.max(y);
    }
    target.0 -= 1;
    (start, target, terrain)
    // use nom::bytes::complete::{is_a, tag};
    // use nom::character::complete::{alpha1, alphanumeric1, char, newline, one_of};
    // use nom::combinator::{map, opt, value};
    // use nom::multi::{many1, separated_list1};
    // use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};

    // separated_list1(newline, tag("asdf"))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    SlopeRight,
    SlopeUp,
    SlopeLeft,
    SlopeDown,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";
        let result = part1(input);
        assert_eq!(result, "94");
    }

    #[test]
    #[ignore = "not done yet"]
    fn test_part2() {
        let input = "\
";
        let result = part2(input);
        assert_eq!(result, "todo");
    }
}
