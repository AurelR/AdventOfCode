type NumTy = i32;
type NumPair = (NumTy, NumTy);
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = std::fs::read_to_string("data/input/input16.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let data = parse_input(input).unwrap().1;
    let grid = reformat_data(data);

    energize_grid(&grid, (0, 0), (1, 0)).to_string()
}

fn part2(input: &str) -> String {
    let data = parse_input(input).unwrap().1;
    let grid = reformat_data(data);

    let mut max = 0;

    for x in 0..grid.x_size {
        max = max.max(energize_grid(&grid, (x, 0), (0, 1)));
        max = max.max(energize_grid(&grid, (x, grid.y_size - 1), (0, -1)));
    }
    for y in 0..grid.y_size {
        max = max.max(energize_grid(&grid, (0, y), (1, 0)));
        max = max.max(energize_grid(&grid, (grid.x_size - 1, y), (-1, 0)));
    }
    max.to_string()
}

fn energize_grid(grid: &Grid, start_pos: NumPair, start_dir: NumPair) -> usize {
    let mut cache = BTreeSet::new();
    let mut energized = BTreeSet::new();
    let mut beams = Vec::new();
    beams.push((start_pos, start_dir));
    while !beams.is_empty() {
        let mut new_beams = Vec::new();
        for beam in beams.into_iter() {
            if !cache.insert(beam) {
                continue;
            }
            let (pos, dir) = beam;
            if let Some(tile) = grid.contents.get(&pos) {
                energized.insert(pos);
                let (new_dir1, new_dir2) = tile.pass_through(dir);
                new_beams.push(((pos.0 + new_dir1.0, pos.1 + new_dir1.1), new_dir1));
                if let Some(new_dir2) = new_dir2 {
                    new_beams.push(((pos.0 + new_dir2.0, pos.1 + new_dir2.1), new_dir2));
                }
            }
        }
        beams = new_beams;
    }
    energized.len()
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<Vec<Tile>>> {
    use nom::branch::alt;
    use nom::character::complete::{char, newline};
    use nom::combinator::value;
    use nom::multi::{many1, separated_list1};

    separated_list1(
        newline,
        many1(alt((
            value(Tile::Empty, char('.')),
            value(Tile::MirrowUpDown, char('/')),
            value(Tile::MirrorDownUp, char('\\')),
            value(Tile::SplitterVertical, char('|')),
            value(Tile::SplitterHorizontal, char('-')),
        ))),
    )(input)
}

fn reformat_data(data: Vec<Vec<Tile>>) -> Grid {
    let x_size = data[0].len() as NumTy;
    let y_size = data.len() as NumTy;
    let contents = data
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .map(move |(x, tile)| ((x as NumTy, y as NumTy), tile))
        })
        .collect();
    Grid {
        contents,
        x_size,
        y_size,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    MirrowUpDown,
    MirrorDownUp,
    SplitterVertical,
    SplitterHorizontal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    contents: BTreeMap<NumPair, Tile>,
    x_size: NumTy,
    y_size: NumTy,
}

impl Tile {
    fn pass_through(&self, direction_in: NumPair) -> (NumPair, Option<NumPair>) {
        match (self, direction_in) {
            (Tile::Empty, _) => (direction_in, None),
            (Tile::MirrowUpDown, (1, 0)) => ((0, -1), None),
            (Tile::MirrowUpDown, (-1, 0)) => ((0, 1), None),
            (Tile::MirrowUpDown, (0, 1)) => ((-1, 0), None),
            (Tile::MirrowUpDown, (0, -1)) => ((1, 0), None),
            (Tile::MirrorDownUp, (1, 0)) => ((0, 1), None),
            (Tile::MirrorDownUp, (-1, 0)) => ((0, -1), None),
            (Tile::MirrorDownUp, (0, 1)) => ((1, 0), None),
            (Tile::MirrorDownUp, (0, -1)) => ((-1, 0), None),
            (Tile::SplitterVertical, (_, 0)) => ((0, 1), Some((0, -1))),
            (Tile::SplitterVertical, _) => (direction_in, None),
            (Tile::SplitterHorizontal, (0, _)) => ((1, 0), Some((-1, 0))),
            (Tile::SplitterHorizontal, _) => (direction_in, None),
            _ => panic!("Invalid direction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;
        let result = part1(input);
        assert_eq!(result, "46");
    }

    #[test]
    fn test_part2() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;
        let result = part2(input);
        assert_eq!(result, "51");
    }
}
