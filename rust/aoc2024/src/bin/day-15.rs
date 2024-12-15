use std::{collections::BTreeMap, fmt::Display};

//use itertools::Itertools;
//use nom::character::complete::i32 as int;

type NumTy = i32;
type Pos = (NumTy, NumTy);
type Map = BTreeMap<Pos, Cell>;
type Instructions = Vec<Pos>;

fn main() {
    let input = std::fs::read_to_string("data/input/input15.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (mut pos, mut map, instructions) = parse(input);
    for instr in instructions {
        let new_pos = (pos.0 + instr.0, pos.1 + instr.1);
        if map.get(&new_pos).is_none_or(|c| c.is_wall()) {
            continue;
        }

        if map.get(&new_pos).is_some_and(|c| c.is_empty()) {
            pos = new_pos;
            continue;
        }

        if let Some(empty_pos) = find_empty(new_pos, instr, &map) {
            *map.get_mut(&new_pos).unwrap() = Cell::Empty;
            *map.get_mut(&empty_pos).unwrap() = Cell::Box;
            pos = new_pos;
        }
    }
    score(&map).to_string()
}

fn find_empty(mut pos: Pos, dir: Pos, map: &Map) -> Option<Pos> {
    loop {
        match map.get(&pos) {
            Some(Cell::Wall) => return None,
            None => todo!("Should not happen"),
            Some(Cell::Empty) => return Some(pos),
            Some(Cell::Box) => pos = (pos.0 + dir.0, pos.1 + dir.1),
        }
    }
}

fn score(map: &Map) -> NumTy {
    map.iter()
        .filter_map(|(pos, c)| {
            if c.is_box() {
                Some(pos.0 + 100 * pos.1)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> String {
    let _data = parse(input);
    "".to_string()
}

fn parse(input: &str) -> (Pos, Map, Instructions) {
    let (warehouse_str, instruction_str) = input.split_once("\n\n").unwrap();
    let mut start = (0, 0);
    let mut map = Map::new();
    for (y, line) in warehouse_str.lines().enumerate() {
        let y = y as NumTy;
        for (x, c) in line.chars().enumerate() {
            let x = x as NumTy;
            match c {
                '.' => {
                    map.insert((x, y), Cell::Empty);
                }
                '#' => {
                    map.insert((x, y), Cell::Wall);
                }
                'O' => {
                    map.insert((x, y), Cell::Box);
                }
                '@' => {
                    start = (x, y);
                    map.insert((x, y), Cell::Empty);
                }
                _ => panic!("Invalid char \"{c}\" at ({x},{y})"),
            }
        }
    }

    let mut instructions = Instructions::new();
    for c in instruction_str.chars() {
        match c {
            '\n' => continue,
            '^' => instructions.push((0, -1)),
            'v' => instructions.push((0, 1)),
            '>' => instructions.push((1, 0)),
            '<' => instructions.push((-1, 0)),
            _ => panic!("Invalid char \"{c}\" in move instructions"),
        }
    }

    (start, map, instructions)
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Empty,
    Wall,
    Box,
}

impl Cell {
    fn is_wall(&self) -> bool {
        match self {
            Cell::Wall => true,
            _ => false,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Cell::Empty => true,
            _ => false,
        }
    }

    fn is_box(&self) -> bool {
        match self {
            Cell::Box => true,
            _ => false,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Wall => write!(f, "#"),
            Cell::Box => write!(f, "O"),
        }
    }
}

#[allow(dead_code)]
fn debug_print(pos: Pos, map: &Map) {
    let (maxx, maxy) = map
        .keys()
        .fold((0, 0), |(x, y), (xn, yn)| (x.max(*xn), y.max(*yn)));
    for y in 0..=maxy {
        for x in 0..=maxx {
            if (x, y) == pos {
                print!("@");
            } else {
                print!("{}", map[&(x, y)]);
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_a() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

        let result = part1(input);
        assert_eq!("2028", result);
    }

    #[test]
    fn test_part1_b() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

        let result = part1(input);
        assert_eq!("10092", result);
    }

    #[test]
    fn test_part2() {
        let input = "
";

        let result = part2(input);
        assert_eq!("000000", result);
    }
}
