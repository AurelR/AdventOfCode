use pathfinding::directed::astar;
use std::collections::BTreeSet;

type NumTy = i32;
type Pos = (NumTy, NumTy);
type Map = BTreeSet<Pos>;

fn main() {
    let input = std::fs::read_to_string("data/input/input16.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (start, end, walls) = parse(input);
    let (_path, costs) = astar::astar(
        &(start, (1, 0)),
        |&(pos, dir)| {
            let mut succ = Vec::new();
            let straight = (pos.0 + dir.0, pos.1 + dir.1);
            if !walls.contains(&straight) {
                succ.push(((straight, dir), 1));
            }
            let dir_left = rotate_left(dir);
            let left = (pos.0 + dir_left.0, pos.1 + dir_left.1);
            if !walls.contains(&left) {
                succ.push(((left, dir_left), 1001));
            }
            let dir_right = rotate_right(dir);
            let right = (pos.0 + dir_right.0, pos.1 + dir_right.1);
            if !walls.contains(&right) {
                succ.push(((right, dir_right), 1001));
            }

            succ
        },
        |(pos, _dir)| (pos.0.abs_diff(end.0) + pos.1.abs_diff(end.1)) as NumTy,
        |(pos, _dir)| *pos == end,
    )
    .unwrap();

    costs.to_string()
}

fn part2(input: &str) -> String {
    let (start, end, walls) = parse(input);
    let solutions = astar::astar_bag(
        &(start, (1, 0)),
        |&(pos, dir)| {
            let mut succ = Vec::new();
            let straight = (pos.0 + dir.0, pos.1 + dir.1);
            if !walls.contains(&straight) {
                succ.push(((straight, dir), 1));
            }
            let dir_left = rotate_left(dir);
            let left = (pos.0 + dir_left.0, pos.1 + dir_left.1);
            if !walls.contains(&left) {
                succ.push(((left, dir_left), 1001));
            }
            let dir_right = rotate_right(dir);
            let right = (pos.0 + dir_right.0, pos.1 + dir_right.1);
            if !walls.contains(&right) {
                succ.push(((right, dir_right), 1001));
            }

            succ
        },
        |(pos, _dir)| (pos.0.abs_diff(end.0) + pos.1.abs_diff(end.1)) as NumTy,
        |(pos, _dir)| *pos == end,
    )
    .unwrap()
    .0;

    let visited = solutions
        .flat_map(|solution| solution.into_iter().map(|(pos, _dir)| pos))
        .collect::<BTreeSet<_>>();
    visited.len().to_string()
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

fn rotate_right(dir: Pos) -> Pos {
    (-dir.1, dir.0)
}

fn rotate_left(dir: Pos) -> Pos {
    (dir.1, -dir.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_a() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

        let result = part1(input);
        assert_eq!("7036", result);
    }

    #[test]
    fn test_part1_b() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

        let result = part1(input);
        assert_eq!("11048", result);
    }

    #[test]
    fn test_part2_a() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

        let result = part2(input);
        assert_eq!("45", result);
    }

    #[test]
    fn test_part2_b() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

        let result = part2(input);
        assert_eq!("64", result);
    }
}
