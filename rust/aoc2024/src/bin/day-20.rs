use pathfinding::directed::astar;
use std::collections::{BTreeMap, BTreeSet};

type NumTy = i32;
type Pos = (NumTy, NumTy);
type Map = BTreeSet<Pos>;
type PathMap = BTreeMap<Pos, usize>;

fn main() {
    let input = std::fs::read_to_string("data/input/input20.txt").unwrap();
    let result1 = part1(&input, 100);
    println!("{}", result1);
    let result2 = part2(&input, 100);
    println!("{}", result2);
}

fn part1(input: &str, limit: usize) -> String {
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
            if let Some(cc) = path_map.get(&candidate).filter(|cc| **cc > cost + 2) {
                *result.entry(cc - cost - 2).or_default() += 1;
            }
        }
    }
    result.into_iter().filter_map(|(k,v)| if k >= limit { Some(v)} else {None}).sum::<usize>().to_string()
}

fn part2(input: &str, limit: usize) -> String {
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
    for (cost, &pos) in path.iter().enumerate() {
        for (&candiate, &candidate_cost) in path_map.iter() {
            if candidate_cost <= cost + 2 {
                continue;
            }
            let diff = (pos.0.abs_diff(candiate.0) + pos.1.abs_diff(candiate.1)) as usize;
            if 2 <= diff && diff <= 20 {
                if is_wall_connected(pos, candiate, &walls) {
                    *result.entry(candidate_cost - cost - diff).or_default() += 1
                }
             }
        }
    }
    result.into_iter().filter_map(|(k,v)| if k >= limit { Some(v)} else {None}).sum::<usize>().to_string()
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

fn is_wall_connected(start: Pos, target: Pos, walls: &Map) -> bool {
    let path = astar::astar(
        &start,
        |&p| {
            [
                ((p.0, p.1 + 1), 1),
                ((p.0, p.1 - 1), 1),
                ((p.0 + 1, p.1), 1),
                ((p.0 - 1, p.1), 1),
            ]
            .into_iter()
            .filter(|(p, _c)| *p == target || walls.contains(&p))
            .collect::<Vec<_>>()
        },
        |&p| (p.0.abs_diff(target.0) + p.1.abs_diff(target.1)) as NumTy,
        |&p| p == target,
    );
    path.is_some()
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

        let result = part1(input, 2);
        assert_eq!("44", result);
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

        let result = part2(input, 50);
        assert_eq!("285", result);
    }
}
