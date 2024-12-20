use pathfinding::directed::astar;
use std::collections::{BTreeMap, BTreeSet};

type NumTy = i32;
type Pos = (NumTy, NumTy);
type Map = BTreeSet<Pos>;
type PathMap = BTreeMap<Pos, usize>;

fn main() {
    let input = std::fs::read_to_string("data/input/input20.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (start, end, walls) = parse(input);
    let (path, _costs) = astar::astar(
        &start,
        |&pos| {
            [
                ((pos.0, pos.1 + 1), 1),
                ((pos.0, pos.1 - 1), 1),
                ((pos.0 + 1, pos.1), 1),
                ((pos.0 - 1, pos.1), 1),
            ]
            .into_iter()
            .filter(|(pos, _c)| !walls.contains(&pos))
            .collect::<Vec<_>>()
        },
        |pos| (pos.0.abs_diff(end.0) + pos.1.abs_diff(end.1)) as NumTy,
        |pos| *pos == end,
    )
    .unwrap();

    let mut result = BTreeMap::<usize, usize>::new();
    let path_map = PathMap::from_iter(path.iter().enumerate().map(|(c, p)| (*p, c)));
    for (cost, pos) in path.iter().enumerate() {
        let candiates = [
            (pos.0, pos.1 + 2),
            (pos.0, pos.1 - 2),
            (pos.0 + 2, pos.1),
            (pos.0 - 2, pos.1),
        ];
        for candidate in candiates {
            if let Some(cc) = path_map.get(&candidate).filter(|cc| **cc > cost && **cc - cost > 2) {
                *result.entry(cc - cost - 2).or_default() += 1;
            }
        }
    }
    result.into_iter().filter_map(|(k,v)| if k >= 100 { Some(v)} else {None}).sum::<usize>().to_string()
}

fn part2(_input: &str) -> String {
    todo!()
}

fn parse(input: &str) -> (Pos, Pos, Map) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut map = Map::new();

    for (y, line) in input.lines().enumerate() {
        let y = y as NumTy;
        for (x, c) in line.chars().enumerate() {
            let x = x as NumTy;
            match c {
                '.' => {
                    continue;
                }
                '#' => {
                    map.insert((x, y));
                }
                'E' => end = (x, y),
                'S' => {
                    start = (x, y);
                }
                _ => panic!("Invalid char \"{c}\" at ({x},{y})"),
            }
        }
    }

    (start, end, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

        let result = part1(input);
        assert_eq!("0", result);
    }

    #[test]
    fn test_part2() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

        let result = part2(input);
        assert_eq!("000000", result);
    }
}
