use pathfinding::directed::astar;
use std::{collections::BTreeSet, ops::Range};

use nom::character::complete::i32 as parse_int;
type NumTy = i32;
type Pos = (NumTy, NumTy);
type Map = BTreeSet<Pos>;

fn main() {
    let input = std::fs::read_to_string("data/input/input18.txt").unwrap();
    let result1 = part1(&input, 1024, 0..71, 0..71);
    println!("{}", result1);
    let result2 = part2(&input, 1024, 0..71, 0..71);
    println!("{}", result2);
}

fn part1(input: &str, steps: usize, dimx: Range<NumTy>, dimy: Range<NumTy>) -> String {
    let bytes = parse(input).unwrap().1;
    let corupted = bytes.into_iter().take(steps).collect::<Map>();
    let start = (0, 0);
    let end = (dimx.end - 1, dimy.end - 1);
    let (_path, costs) = astar::astar(
        &start,
        |&pos| {
            [
                ((pos.0, pos.1 + 1), 1),
                ((pos.0, pos.1 - 1), 1),
                ((pos.0 + 1, pos.1), 1),
                ((pos.0 - 1, pos.1), 1),
            ]
            .into_iter()
            .filter(|(pos, _c)| {
                dimx.contains(&pos.0) && dimy.contains(&pos.1) && !corupted.contains(&pos)
            })
            .collect::<Vec<_>>()
        },
        |pos| (pos.0.abs_diff(end.0) + pos.1.abs_diff(end.1)) as NumTy,
        |pos| *pos == end,
    )
    .unwrap();

    costs.to_string()
}

fn part2(input: &str, steps: usize, dimx: Range<NumTy>, dimy: Range<NumTy>) -> String {
    let bytes = parse(input).unwrap().1;
    let start = (0, 0);
    let end = (dimx.end - 1, dimy.end - 1);

    let mut result = (0, 0);
    for s in steps..=bytes.len() {
        let corupted = bytes.iter().take(s).copied().collect::<Map>();
        assert!(corupted.len() == s);
        let path = astar::astar(
            &start,
            |&pos| {
                [
                    ((pos.0, pos.1 + 1), 1),
                    ((pos.0, pos.1 - 1), 1),
                    ((pos.0 + 1, pos.1), 1),
                    ((pos.0 - 1, pos.1), 1),
                ]
                .into_iter()
                .filter(|(pos, _c)| {
                    dimx.contains(&pos.0) && dimy.contains(&pos.1) && !corupted.contains(&pos)
                })
                .collect::<Vec<_>>()
            },
            |pos| (pos.0.abs_diff(end.0) + pos.1.abs_diff(end.1)) as NumTy,
            |pos| *pos == end,
        );
        if path.is_none() {
            result = bytes[s - 1];
            break;
        }
    }

    format!("{},{}", result.0, result.1)
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Pos>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::newline;
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;

    separated_list1(newline, separated_pair(parse_int, tag(","), parse_int))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

        let result = part1(input, 12, 0..7, 0..7);
        assert_eq!("22", result);
    }

    #[test]
    fn test_part2() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

        let result = part2(input, 12, 0..7, 0..7);
        assert_eq!("6,1", result);
    }
}
