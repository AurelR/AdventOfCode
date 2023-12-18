type NumTy = i32;
use nom::{character::complete::i32 as num_parser, sequence::delimited};
type NumPair = (NumTy, NumTy);
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = std::fs::read_to_string("data/input/input18.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let data = parse_input(input).unwrap().1;
    let mut points = Vec::new();
    let mut current_pos = (0, 0);
    let mut border = 0;
    points.push(current_pos);
    for (dir, _, len, _) in data {
        border += len;
        current_pos = match dir {
            Direction::Up => (current_pos.0, current_pos.1 - len),
            Direction::Down => (current_pos.0, current_pos.1 + len),
            Direction::Left => (current_pos.0 - len, current_pos.1),
            Direction::Right => (current_pos.0 + len, current_pos.1),
        };
        points.push(current_pos);
    }
    // Shoelace formula
    let area = points
        .windows(2)
        .map(|p| p[0].0 * p[1].1 - p[1].0 * p[0].1)
        .sum::<NumTy>()
        / 2;
    // Picks Theorem
    let result = area.abs() + border / 2 + 1;
    result.to_string()
}

fn part2(input: &str) -> String {
    let _data = parse_input(input).unwrap().1;
    "".to_string()
}

#[allow(unused_imports)]
fn parse_input(input: &str) -> nom::IResult<&str, Vec<(Direction, &str, NumTy, &str)>> {
    use nom::branch::alt;
    use nom::bytes::complete::{is_a, tag};
    use nom::character::complete::{alphanumeric1, char, newline, space1};
    use nom::combinator::{map, value};
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{preceded, separated_pair, tuple};

    separated_list1(
        newline,
        tuple((
            alt((
                value(Direction::Up, char('U')),
                value(Direction::Down, char('D')),
                value(Direction::Left, char('L')),
                value(Direction::Right, char('R')),
            )),
            space1,
            num_parser,
            delimited(tag(" (#"), alphanumeric1, tag(")")),
        )),
    )(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// fn reformat_data(data: Vec<Vec<Tile>>) -> Grid {
//     let x_size = data[0].len() as NumTy;
//     let y_size = data.len() as NumTy;
//     let contents = data
//         .into_iter()
//         .enumerate()
//         .flat_map(|(y, line)| {
//             line.into_iter()
//                 .enumerate()
//                 .map(move |(x, tile)| ((x as NumTy, y as NumTy), tile))
//         })
//         .collect();
//     Grid {
//         contents,
//         x_size,
//         y_size,
//     }
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct Grid {
//     contents: BTreeMap<NumPair, Tile>,
//     x_size: NumTy,
//     y_size: NumTy,
// }

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";
        let result = part1(input);
        assert_eq!(result, "62");
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
