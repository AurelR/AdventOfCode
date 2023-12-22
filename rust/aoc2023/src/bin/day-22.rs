type NumTy = i32;
use itertools::Itertools;
use nom::character::complete::i32 as num_parser;
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::RangeInclusive,
};

fn main() {
    let input = std::fs::read_to_string("data/input/input22.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let mut bricks = parse_input(input);
    let dimensions = bricks.iter().fold(
        ((NumTy::MAX, NumTy::MIN), (NumTy::MAX, NumTy::MIN)),
        |((xmin, xmax), (ymin, ymax)), brick| {
            (
                (xmin.min(*brick.x.start()), xmax.max(*brick.x.end())),
                (ymin.min(*brick.y.start()), ymax.max(*brick.y.end())),
            )
        },
    );

    bricks.sort_by_key(|b| {
        (
            *b.z.start(),
            *b.z.end(),
            *b.y.start(),
            *b.y.end(),
            *b.x.start(),
            *b.x.end(),
        )
    });
    let mut highest = BTreeMap::from_iter(
        (dimensions.0 .0..=dimensions.0 .1)
            .cartesian_product(dimensions.1 .0..=dimensions.1 .1)
            .map(|pos| (pos, (0, 0))),
    );
    let mut supported = bricks
        .iter()
        .map(|b| (b.id, BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    let mut supporting = bricks
        .iter()
        .map(|b| (b.id, BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    for brick in &mut bricks {
        let mut height = 0;
        for pos in brick.y.clone().cartesian_product(brick.x.clone()) {
            height = height.max(highest[&pos].0);
        }
        let shift = brick.z.start() - height - 1;
        brick.z = brick.z.start() - shift..=brick.z.end() - shift;
        for pos in brick.y.clone().cartesian_product(brick.x.clone()) {
            let height_entry = highest.get_mut(&pos).unwrap();
            if height_entry.0 == height {
                supported
                    .entry(brick.id)
                    .or_insert(BTreeSet::new())
                    .insert(height_entry.1);
                supporting
                    .entry(height_entry.1)
                    .or_insert(BTreeSet::new())
                    .insert(brick.id);
            }

            *height_entry = (*brick.z.end(), brick.id);
        }
    }

    bricks
        .into_iter()
        .filter(|b| {
            let upper = &supporting[&b.id];
            if upper.is_empty() {
                return true;
            }
            upper.iter().all(|bb| supported[bb].len() > 1)
        })
        .count()
        .to_string()
}

fn part2(input: &str) -> String {
    let _data = parse_input(input);
    "".to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    id: usize,
    x: RangeInclusive<NumTy>,
    y: RangeInclusive<NumTy>,
    z: RangeInclusive<NumTy>,
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let (x1, y1, z1, x2, y2, z2) = parse_brick_data(line).unwrap().1;
            Brick {
                id: id + 1,
                x: x1..=x2,
                y: y1..=y2,
                z: z1..=z2,
            }
        })
        .collect()
}

fn parse_brick_data(input: &str) -> nom::IResult<&str, (NumTy, NumTy, NumTy, NumTy, NumTy, NumTy)> {
    use nom::character::complete::char;
    use nom::sequence::{preceded, tuple};
    tuple((
        num_parser,
        preceded(char(','), num_parser),
        preceded(char(','), num_parser),
        preceded(char('~'), num_parser),
        preceded(char(','), num_parser),
        preceded(char(','), num_parser),
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";
        let result = part1(input);
        assert_eq!(result, "5");
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
