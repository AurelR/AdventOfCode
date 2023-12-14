type NumTy = i32;
type NumPair = (NumTy, NumTy);
use std::collections::BTreeMap;

fn main() {
    let input = std::fs::read_to_string("data/input/input14.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let data = parse_input(input).unwrap().1;
    let grid = reformat_data(data);
    let mut new_grid = grid.clone();
    let mut load = 0;
    for x in 0..grid.x_size {
        for y in 0..grid.y_size {
            match new_grid.contents.get(&(x, y)) {
                Some(Location::SquareRock) => continue,
                Some(Location::RoundedRock) => {
                    load += grid.y_size - y;
                    continue;
                }
                Some(Location::Empty) => {}
                None => unreachable!("Outside grid"),
            }

            if let Some(test_y) = (y + 1..grid.y_size)
                .find(|fy| *new_grid.contents.get(&(x, *fy)).unwrap() != Location::Empty)
            {
                let test_pos = new_grid.contents.get_mut(&(x, test_y)).unwrap();
                if *test_pos == Location::RoundedRock {
                    *test_pos = Location::Empty;
                    load += grid.y_size - y;
                    *new_grid.contents.get_mut(&(x, y)).unwrap() = Location::RoundedRock;
                }
            }
        }
    }
    dbg!(new_grid);
    load.to_string()
}

fn part2(input: &str) -> String {
    let _data = parse_input(input).unwrap().1;
    "".to_string()
}

#[allow(unused_imports)]
fn parse_input(input: &str) -> nom::IResult<&str, Vec<Vec<Location>>> {
    use nom::branch::alt;
    use nom::bytes::complete::{is_a, tag};
    use nom::character::complete::{char, newline, space1};
    use nom::combinator::{map, value};
    use nom::multi::{many1, separated_list1};
    use nom::sequence::separated_pair;

    separated_list1(
        newline,
        many1(alt((
            value(Location::Empty, char('.')),
            value(Location::SquareRock, char('#')),
            value(Location::RoundedRock, char('O')),
        ))),
    )(input)
}

fn reformat_data(data: Vec<Vec<Location>>) -> Grid {
    let x_size = data[0].len() as NumTy;
    let y_size = data.len() as NumTy;
    let contents = data
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .map(move |(x, loc)| ((x as NumTy, y as NumTy), loc))
        })
        .collect();
    Grid {
        contents,
        x_size,
        y_size,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Location {
    Empty,
    SquareRock,
    RoundedRock,
}

#[derive(Debug, Clone)]
struct Grid {
    contents: BTreeMap<NumPair, Location>,
    x_size: NumTy,
    y_size: NumTy,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
        let result = part1(input);
        assert_eq!(result, "136");
    }

    #[test]
    #[ignore = "not ready yet"]
    fn test_part2() {
        let input = "\
        ";
        let result = part2(input);
        assert_eq!(result, "todo");
    }
}
