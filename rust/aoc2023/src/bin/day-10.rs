use std::collections::BTreeMap;

type NumTy = i32;
type NumPair = (NumTy, NumTy);

fn main() {
    let input = std::fs::read_to_string("data/input/input10.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let data = parse_input(input).unwrap().1;
    let map = convert_input(data);
    let start_pos = *map
        .iter()
        .find(|(_pos, pipe)| if let Pipe::Start = pipe { true } else { false })
        .unwrap()
        .0;
    
    let mut dir = find_starting_direction(start_pos, &map);
    let mut pos = start_pos;
    let mut count = 1;

    loop {
        pos = (pos.0 + dir.0,pos.1 + dir.1);
        let pipe = map.get(&pos).unwrap();
        if let Pipe::Start = pipe {
            break;
        };
        count += 1;
        dir = pipe.follow(dir).unwrap();
    }

    (count / 2).to_string()
}

fn part2(input: &str) -> String {
    let _data = parse_input(input).unwrap().1;
    "".to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Ground,
    Start,
}

impl Pipe {
    fn from_str(c: char) -> Pipe {
        match c {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthToEast,
            'J' => Pipe::NorthToWest,
            '7' => Pipe::SouthToWest,
            'F' => Pipe::SouthToEast,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!("Invalid char {c}"),
        }
    }

    fn follow(&self, direction: NumPair) -> Option<NumPair> {
        match (self, direction) {
            (Pipe::Vertical, (0, 1)) => Some((0, 1)),
            (Pipe::Vertical, (0, -1)) => Some((0, -1)),
            (Pipe::Horizontal, (1, 0)) => Some((1, 0)),
            (Pipe::Horizontal, (-1, 0)) => Some((-1, 0)),
            (Pipe::NorthToEast, (0, 1)) => Some((1, 0)),
            (Pipe::NorthToEast, (-1, 0)) => Some((0, -1)),
            (Pipe::NorthToWest, (0, 1)) => Some((-1, 0)),
            (Pipe::NorthToWest, (1, 0)) => Some((0, -1)),
            (Pipe::SouthToWest, (0, -1)) => Some((-1, 0)),
            (Pipe::SouthToWest, (1, 0)) => Some((0, 1)),
            (Pipe::SouthToEast, (0, -1)) => Some((1, 0)),
            (Pipe::SouthToEast, (-1, 0)) => Some((0, 1)),
            _ => None,
        }
    }
}

fn find_starting_direction(start_pos: NumPair, map: &BTreeMap<NumPair, Pipe>) -> NumPair {
    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let pos = (start_pos.0 + dir.0, start_pos.1 + dir.1);
        if map.get(&pos).and_then(|pipe| pipe.follow(dir)).is_some() {
            return dir;
        }
    }
    panic!("No direction to start");
}

#[allow(unused_imports)]
fn parse_input(input: &str) -> nom::IResult<&str, Vec<Vec<Pipe>>> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, alphanumeric1, newline, none_of, space1};
    use nom::combinator::map;
    use nom::combinator::value;
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{delimited, preceded, separated_pair, terminated};

    separated_list1(newline, many1(map(none_of("\n"), Pipe::from_str)))(input)
}

fn convert_input(data: Vec<Vec<Pipe>>) -> BTreeMap<(NumTy, NumTy), Pipe> {
    data.into_iter()
        .enumerate()
        .flat_map(|(y, col)| {
            col.into_iter()
                .enumerate()
                .map(move |(x, p)| ((x as NumTy, y as NumTy), p))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1a() {
        let input = "\
.....
.S-7.
.|.|.
.L-J.
.....
";
        let result = part1(input);
        assert_eq!(result, "4");
    }

    #[test]
    fn test_part1b() {
        let input = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
        let result = part1(input);
        assert_eq!(result, "8");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_part2() {
        let input = "\
";
        let result = part2(input);
        assert_eq!(result, "todo");
    }
}
