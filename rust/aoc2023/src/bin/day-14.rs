type NumTy = i32;
type NumPair = (NumTy, NumTy);
use std::collections::{BTreeMap, HashMap};

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
    grid.tilt_north().load().to_string()
}

fn part2(input: &str) -> String {
    let data = parse_input(input).unwrap().1;
    let mut grid = reformat_data(data);
    let mut cache = HashMap::new();
    let mut n0 = 0;
    let mut n1 = 0;
    for c in 0.. {
        let new_grid = grid.cycle();
        cache.insert(grid, c);
        if let Some(c0) = cache.get(&new_grid) {
            n0 = *c0;
            n1 = c;
            break;
        }
        grid = new_grid;
    }

    let a = 1_000_000_000 - n0;
    let b = n1 - n0 + 1;
    let c = a % b;
    let n = c + n0;
    let result = cache
        .iter()
        .find_map(|(k, v)| if *v == n { Some(k) } else { None });
    result.unwrap().load().to_string()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Location {
    Empty,
    SquareRock,
    RoundedRock,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    contents: BTreeMap<NumPair, Location>,
    x_size: NumTy,
    y_size: NumTy,
}

impl Grid {
    fn cycle(&self) -> Grid {
        self.tilt_north().tilt_west().tilt_south().tilt_east()
    }
    fn load(&self) -> NumTy {
        let mut load = 0;
        for x in 0..self.x_size {
            for y in 0..self.y_size {
                match self.contents.get(&(x, y)) {
                    Some(Location::SquareRock) => continue,
                    Some(Location::RoundedRock) => {
                        load += self.y_size - y;
                        continue;
                    }
                    Some(Location::Empty) => {}
                    None => unreachable!("Outside grid"),
                }
            }
        }
        load
    }

    fn tilt_north(&self) -> Grid {
        let mut tilted_grid = self.clone();
        for x in 0..self.x_size {
            for y in 0..self.y_size {
                match tilted_grid.contents.get(&(x, y)) {
                    Some(Location::Empty) => {}
                    Some(Location::SquareRock) => continue,
                    Some(Location::RoundedRock) => continue,
                    None => unreachable!("Outside grid"),
                }

                if let Some(test_y) = (y + 1..self.y_size)
                    .find(|fy| *tilted_grid.contents.get(&(x, *fy)).unwrap() != Location::Empty)
                {
                    let test_pos = tilted_grid.contents.get_mut(&(x, test_y)).unwrap();
                    if *test_pos == Location::RoundedRock {
                        *test_pos = Location::Empty;
                        *tilted_grid.contents.get_mut(&(x, y)).unwrap() = Location::RoundedRock;
                    }
                }
            }
        }
        tilted_grid
    }

    fn tilt_west(&self) -> Grid {
        let mut tilted_grid = self.clone();
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                match tilted_grid.contents.get(&(x, y)) {
                    Some(Location::Empty) => {}
                    Some(Location::SquareRock) => continue,
                    Some(Location::RoundedRock) => continue,
                    None => unreachable!("Outside grid"),
                }

                if let Some(test_x) = (x + 1..self.x_size)
                    .find(|fx| *tilted_grid.contents.get(&(*fx, y)).unwrap() != Location::Empty)
                {
                    let test_pos = tilted_grid.contents.get_mut(&(test_x, y)).unwrap();
                    if *test_pos == Location::RoundedRock {
                        *test_pos = Location::Empty;
                        *tilted_grid.contents.get_mut(&(x, y)).unwrap() = Location::RoundedRock;
                    }
                }
            }
        }
        tilted_grid
    }

    fn tilt_south(&self) -> Grid {
        let mut tilted_grid = self.clone();
        for x in 0..self.x_size {
            for y in (0..self.y_size).rev() {
                match tilted_grid.contents.get(&(x, y)) {
                    Some(Location::Empty) => {}
                    Some(Location::SquareRock) => continue,
                    Some(Location::RoundedRock) => continue,
                    None => unreachable!("Outside grid"),
                }

                if let Some(test_y) = (0..y)
                    .rev()
                    .find(|fy| *tilted_grid.contents.get(&(x, *fy)).unwrap() != Location::Empty)
                {
                    let test_pos = tilted_grid.contents.get_mut(&(x, test_y)).unwrap();
                    if *test_pos == Location::RoundedRock {
                        *test_pos = Location::Empty;
                        *tilted_grid.contents.get_mut(&(x, y)).unwrap() = Location::RoundedRock;
                    }
                }
            }
        }
        tilted_grid
    }

    fn tilt_east(&self) -> Grid {
        let mut tilted_grid = self.clone();
        for y in 0..self.y_size {
            for x in (0..self.x_size).rev() {
                match tilted_grid.contents.get(&(x, y)) {
                    Some(Location::Empty) => {}
                    Some(Location::SquareRock) => continue,
                    Some(Location::RoundedRock) => continue,
                    None => unreachable!("Outside grid"),
                }

                if let Some(test_x) = (0..x)
                    .rev()
                    .find(|fx| *tilted_grid.contents.get(&(*fx, y)).unwrap() != Location::Empty)
                {
                    let test_pos = tilted_grid.contents.get_mut(&(test_x, y)).unwrap();
                    if *test_pos == Location::RoundedRock {
                        *test_pos = Location::Empty;
                        *tilted_grid.contents.get_mut(&(x, y)).unwrap() = Location::RoundedRock;
                    }
                }
            }
        }
        tilted_grid
    }
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
    fn test_part2() {
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
        let result = part2(input);
        assert_eq!(result, "64");
    }

    #[test]
    fn test_cycle1() {
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
        let expected_input = "\
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
";
        let result = reformat_data(parse_input(input).unwrap().1).cycle();
        let expected = reformat_data(parse_input(expected_input).unwrap().1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_cycle2() {
        let input = "\
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
";

        let expected_input = "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
";
        let result = reformat_data(parse_input(input).unwrap().1).cycle();
        let expected = reformat_data(parse_input(expected_input).unwrap().1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_cycle3() {
        let input = "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
";

        let expected_input = "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
";
        let result = reformat_data(parse_input(input).unwrap().1).cycle();
        let expected = reformat_data(parse_input(expected_input).unwrap().1);
        assert_eq!(result, expected);
    }
}
