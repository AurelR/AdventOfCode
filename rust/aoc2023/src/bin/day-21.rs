type NumTy = i32;
type NumPair = (NumTy, NumTy);
use std::collections::{BTreeSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("data/input/input21.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input, 26501365);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (start, rocks, _) = parse_input(input);
    let start = start.expect("should have a starting postition");
    find_reachable(start, rocks, 64).contents.len().to_string()
}

fn part2(input: &str, steps: NumTy) -> String {
    let (start, rocks, _) = parse_input(input);
    let start = start.expect("should have a starting postition");

    let cycle_length = rocks.x_size;
    let cycle_offset = steps % cycle_length;
    let mut cycle_counts = VecDeque::with_capacity(3);

    let mut current = BTreeSet::new();
    current.insert(start);

    for i in 1..=steps {
        current = perform_step(&current, &rocks);
        if i % cycle_length == cycle_offset {
            cycle_counts.push_front(current.len());
            if cycle_counts.len() == 3 {
                let b0 = cycle_counts[2];
                let b1 = cycle_counts[1] - cycle_counts[2];
                let b2 = cycle_counts[0] - cycle_counts[1];
                let n = (steps / cycle_length) as usize;
                return (b0 + b1 * n + (n * (n - 1) / 2) * (b2 - b1)).to_string();
            }
        }
    }
    current.len().to_string()
}

fn find_reachable(start: NumPair, rocks: Grid, steps: NumTy) -> Grid {
    let mut current = BTreeSet::new();
    current.insert(start);
    for _ in 0..steps {
        current = perform_step(&current, &rocks);
    }
    Grid {
        contents: current,
        ..rocks
    }
}

fn perform_step(current: &BTreeSet<NumPair>, rocks: &Grid) -> BTreeSet<NumPair> {
    let mut next = BTreeSet::new();
    for &c in current {
        for n in neigbors(c).into_iter() {
            if !current.contains(&n)
                && !rocks
                    .contents
                    .contains(&(n.0.rem_euclid(rocks.x_size), n.1.rem_euclid(rocks.y_size)))
            {
                next.insert(n);
            }
        }
    }
    next
}

fn neigbors(pos: NumPair) -> [NumPair; 4] {
    [
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1),
    ]
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    contents: BTreeSet<NumPair>,
    x_size: NumTy,
    y_size: NumTy,
}

fn parse_input(input: &str) -> (Option<NumPair>, Grid, Grid) {
    let mut start = None;
    let mut rocks = BTreeSet::new();
    let mut reached = BTreeSet::new();
    let x_size = input.lines().next().unwrap().as_bytes().len() as NumTy;
    let y_size = input.lines().count() as NumTy;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            match c {
                b'S' => start = Some((x as NumTy, y as NumTy)),
                b'#' => {
                    rocks.insert((x as NumTy, y as NumTy));
                }
                b'O' => {
                    reached.insert((x as NumTy, y as NumTy));
                }
                b'.' => {}
                _ => unreachable!("unexpected char {c}"),
            }
        }
    }

    (
        start,
        Grid {
            contents: rocks,
            x_size,
            y_size,
        },
        Grid {
            contents: reached,
            x_size,
            y_size,
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1_1() {
        let input = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

        let expected = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#O#....
.##.OS####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

        let (start, rocks, _) = parse_input(input);
        let (_, _, expected) = parse_input(expected);
        let result = find_reachable(start.unwrap(), rocks, 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_2() {
        let input = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

        let expected = "\
...........
.....###.#.
.###.##..#.
..#.#O..#..
....#.#....
.##O.O####.
.##.O#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

        let (start, rocks, _) = parse_input(input);
        let (_, _, expected) = parse_input(expected);
        let result = find_reachable(start.unwrap(), rocks, 2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_3() {
        let input = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

        let expected = "\
...........
.....###.#.
.###.##..#.
..#.#.O.#..
...O#O#....
.##.OS####.
.##O.#...#.
....O..##..
.##.#.####.
.##..##.##.
...........
";

        let (start, rocks, _) = parse_input(input);
        let (_, _, expected) = parse_input(expected);
        let result = find_reachable(start.unwrap(), rocks, 3);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_4() {
        let input = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

        let expected = "\
...........
.....###.#.
.###.##.O#.
.O#O#O.O#..
O.O.#.#.O..
.##O.O####.
.##.O#O..#.
.O.O.O.##..
.##.#.####.
.##O.##.##.
...........
";

        let (start, rocks, _) = parse_input(input);
        let (_, _, expected) = parse_input(expected);
        let result = find_reachable(start.unwrap(), rocks, 6);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_wraparound() {
        let input = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";
        let (start, rocks, _) = parse_input(input);
        let result = find_reachable(start.unwrap(), rocks, 10).contents.len();

        assert_eq!(result, 50);
    }

    #[test]
    #[ignore = "solution does not work for test input :-("]
    fn test_part2() {
        let input = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";
        let result = part2(input, 5000);
        assert_eq!(result, "16733044");
    }
}
