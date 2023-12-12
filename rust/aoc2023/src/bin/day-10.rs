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

    let start_dir = find_starting_direction(start_pos, &map);

    let count = std::iter::successors(Some((start_pos, start_dir)), |(pos, dir)| {
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        let pipe = map.get(&new_pos).unwrap();
        if let Pipe::Start = pipe {
            None
        } else {
            let new_dir = pipe.follow(*dir).unwrap();
            Some((new_pos, new_dir))
        }
    })
    .count();

    (count / 2).to_string()
}

fn part2(input: &str) -> String {
    let data = parse_input(input).unwrap().1;
    let map = convert_input(data);
    let start_pos = *map
        .iter()
        .find(|(_pos, pipe)| if let Pipe::Start = pipe { true } else { false })
        .unwrap()
        .0;

    let start_dir = find_starting_direction(start_pos, &map);
    let path = std::iter::successors(Some((start_pos, start_dir)), |(pos, dir)| {
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        let pipe = map.get(&new_pos).unwrap();
        if let Pipe::Start = pipe {
            None
        } else {
            let new_dir = pipe.follow(*dir).unwrap();
            Some((new_pos, new_dir))
        }
    })
    .map(|(pos, _dir)| pos)
    .chain(std::iter::once(start_pos))
    .collect::<Vec<_>>();

    let border = (path.len() - 1) as NumTy;
    // Shoelace formula
    let area = path
        .windows(2)
        .map(|p| p[0].0 * p[1].1 - p[1].0 * p[0].1)
        .sum::<NumTy>();
    // Picks Theorem
    let interior = (area.abs() - border) / 2 + 1;
    interior.to_string()
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
    [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .into_iter()
        .find(|dir| {
            map.get(&(start_pos.0 + dir.0, start_pos.1 + dir.1))
                .and_then(|pipe| pipe.follow(*dir))
                .is_some()
        })
        .expect("No direction to start")
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<Vec<Pipe>>> {
    use nom::character::complete::{newline, none_of};
    use nom::combinator::map;
    use nom::multi::{many1, separated_list1};

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
    fn test_part2a() {
        let input = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";
        let result = part2(input);
        assert_eq!(result, "4");
    }

    #[test]
    fn test_part2b() {
        let input = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
        let result = part2(input);
        assert_eq!(result, "8");
    }

    #[test]
    fn test_part2c() {
        let input = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
        let result = part2(input);
        assert_eq!(result, "10");
    }
}
