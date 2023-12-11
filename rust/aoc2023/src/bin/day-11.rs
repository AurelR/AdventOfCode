type NumTy = usize;
type Position = (NumTy, NumTy);

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("data/input/input11.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let positions = parse_input(input);
    let positions = expand_space(positions, 2);
    sum_all_distance(positions).to_string()
}

fn part2(input: &str) -> String {
    let positions = parse_input(input);
    let positions = expand_space(positions, 1_000_000);
    sum_all_distance(positions).to_string()
}

fn parse_input(input: &str) -> Vec<Position> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(move |(x, c)| match &c {
                    b'#' => Some((x as NumTy, y as NumTy)),
                    b'.' => None,
                    _ => panic!("Unexpected input"),
                })
        })
        .collect()
}

fn expand_space(mut positions: Vec<Position>, expansion: NumTy) -> Vec<Position> {
    let max_x = positions.iter().map(|pos| pos.0).max().unwrap();
    let max_y = positions.iter().map(|pos| pos.1).max().unwrap();
    let mut empty_x = vec![true; max_x + 1];
    let mut empty_y = vec![true; max_y + 1];

    for pos in positions.iter() {
        empty_x[pos.0] = false;
        empty_y[pos.1] = false;
    }

    for (i, x) in empty_x
        .into_iter()
        .enumerate()
        .filter_map(|(col, empty)| if empty { Some(col) } else { None })
        .enumerate()
    {
        for pos in positions.iter_mut() {
            if pos.0 > (x as NumTy) + (i as NumTy) * (expansion - 1) {
                pos.0 += expansion - 1;
            }
        }
    }

    for (i, y) in empty_y
        .into_iter()
        .enumerate()
        .filter_map(|(col, empty)| if empty { Some(col) } else { None })
        .enumerate()
    {
        for pos in positions.iter_mut() {
            if pos.1 > (y as NumTy) + (i as NumTy) * (expansion - 1) {
                pos.1 += expansion - 1;
            }
        }
    }

    positions
}

fn sum_all_distance(positions: Vec<Position>) -> NumTy {
    positions
        .into_iter()
        .tuple_combinations()
        .map(|(p1, p2)| p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_expand_space() {
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
        let positions = parse_input(expected);
        let result = expand_space(parse_input(input), 2);
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
    fn test_part2a() {
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
        let positions = parse_input(input);
        let positions = expand_space(positions, 10);
        let result = sum_all_distance(positions).to_string();
        assert_eq!(result, "1030");
    }

    #[test]
    fn test_part2b() {
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
        let positions = parse_input(input);
        let positions = expand_space(positions, 100);
        let result = sum_all_distance(positions).to_string();
        assert_eq!(result, "8410");
    }
}
