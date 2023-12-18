type NumTy = i64;
use nom::character::complete::i64 as num_parser;
type NumPair = (NumTy, NumTy);

fn main() {
    let input = std::fs::read_to_string("data/input/input18.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let data = parse_input(input).unwrap().1;
    let points = directions_to_points(data.into_iter().map(|(dir, len, _, _)| (dir, len)));
    let (area, _interior, _border) = calculate_areas(&points);
    area.to_string()
}

fn part2(input: &str) -> String {
    let data = parse_input(input).unwrap().1;
    let points = directions_to_points(data.into_iter().map(|(_, _, len, dir)| (dir, len)));
    let (area, _interior, _border) = calculate_areas(&points);
    area.to_string()
}

fn directions_to_points(data: impl Iterator<Item = (Direction, NumTy)>) -> Vec<NumPair> {
    std::iter::once((0, 0))
        .chain(data.scan((0, 0), |current_pos, (dir, len)| {
            *current_pos = match dir {
                Direction::Up => (current_pos.0, current_pos.1 - len),
                Direction::Down => (current_pos.0, current_pos.1 + len),
                Direction::Left => (current_pos.0 - len, current_pos.1),
                Direction::Right => (current_pos.0 + len, current_pos.1),
            };
            Some(*current_pos)
        }))
        .collect()
}

/// First and last point need to be the same point.
/// Returns (area, interior, border)
fn calculate_areas(points: &[NumPair]) -> (NumTy, NumTy, NumTy) {
    let border = points
        .windows(2)
        .map(|p| p[0].0.abs_diff(p[1].0) as NumTy + p[0].1.abs_diff(p[1].1) as NumTy)
        .sum::<NumTy>();
    // Shoelace formula
    let interior = (points
        .windows(2)
        .map(|p| p[0].0 * p[1].1 - p[1].0 * p[0].1)
        .sum::<NumTy>()
        / 2)
    .abs();
    // Picks Theorem
    let area = interior + border / 2 + 1;
    (area, interior, border)
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<(Direction, NumTy, NumTy, Direction)>> {
    use nom::branch::alt;
    use nom::bytes::complete::{tag, take};
    use nom::character::complete::{char, newline, space1};
    use nom::combinator::{map, value};
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, terminated, tuple};

    separated_list1(
        newline,
        tuple((
            alt((
                value(Direction::Up, char('U')),
                value(Direction::Down, char('D')),
                value(Direction::Left, char('L')),
                value(Direction::Right, char('R')),
            )),
            preceded(space1, num_parser),
            preceded(
                tag(" (#"),
                map(take(5usize), |s| NumTy::from_str_radix(s, 16).unwrap()),
            ),
            terminated(
                alt((
                    value(Direction::Up, char('3')),
                    value(Direction::Down, char('1')),
                    value(Direction::Left, char('2')),
                    value(Direction::Right, char('0')),
                )),
                tag(")"),
            ),
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
    fn test_part2() {
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
        let result = part2(input);
        assert_eq!(result, "952408144115");
    }
}
