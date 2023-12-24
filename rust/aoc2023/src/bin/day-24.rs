#![allow(unused_imports)]
type NumTy = f64;
use nom::character::complete::i64 as num_parser;
type NumPair = (NumTy, NumTy);
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = std::fs::read_to_string("data/input/input24.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let data = parse_input(input).unwrap().1;
    dbg!(&data[data.len()-1]);
    let bound_min = 200000000000000.0;
    let bound_max = 400000000000000.0;
    let mut total = 0;
    for a in 0..data.len() {
        for b in a + 1..data.len() {
            let int = data[a].intersection_2d(&data[b]);
            if let Some((p, d)) = int {
                if bound_min <= p.0
                    && p.0 <= bound_max
                    && bound_min <= p.1
                    && p.1 <= bound_max
                    && d.0.signum() >= 0.0
                    && d.1.signum() >= 0.0
                {
                    total += 1;
                }
            }
        }
    }
    total.to_string()
}

fn part2(input: &str) -> String {
    let _data = parse_input(input).unwrap().1;
    "".to_string()
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<Hailstone>> {
    use nom::bytes::complete::{is_a, tag};
    use nom::character::complete::{alpha1, alphanumeric1, char, newline, one_of, space1};
    use nom::combinator::{map, opt, value};
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};

    separated_list1(
        newline,
        map(
            separated_pair(
                tuple((
                    num_parser,
                    preceded(tag(","), preceded(space1, num_parser)),
                    preceded(tag(","), preceded(space1, num_parser)),
                )),
                tag(" @"),
                tuple((
                    preceded(space1, num_parser),
                    preceded(tag(","), preceded(space1, num_parser)),
                    preceded(tag(","), preceded(space1, num_parser)),
                )),
            ),
            |((px, py, pz), (vx, vy, vz))| Hailstone {
                position: Vec3 {
                    x: px as f64,
                    y: py as f64,
                    z: pz as f64,
                },
                velocity: Vec3 {
                    x: vx as f64,
                    y: vy as f64,
                    z: vz as f64,
                },
            },
        ),
    )(input)
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec3 {
    x: NumTy,
    y: NumTy,
    z: NumTy,
}

#[derive(Debug, Clone, PartialEq)]
struct Hailstone {
    position: Vec3,
    velocity: Vec3,
}

impl Hailstone {
    fn intersection_2d(&self, other: &Hailstone) -> Option<(NumPair, NumPair)> {
        let det = self.velocity.y * other.velocity.x - self.velocity.x * other.velocity.y;
        let b_x = other.position.x - self.position.x;
        let b_y = other.position.y - self.position.y;
        let d1 = b_y * other.velocity.x - b_x * other.velocity.y;
        let d2 = self.velocity.x * b_y - self.velocity.y * b_x;
        if det == 0.0 {
            return None;
        }
        let u = d1 / det;
        let v = d2 / det;
        let x = self.velocity.x * u + self.position.x;
        let y = self.velocity.y * u + self.position.y;
        Some(((x, y), (u, v)))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";
        let result = part1(input);
        assert_eq!(result, "2");
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
