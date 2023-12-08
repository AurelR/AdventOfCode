use std::collections::BTreeMap;

fn main() {
    let input = std::fs::read_to_string("data/input/input08.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (directions, map) = parse_input(input).unwrap().1;

    directions
        .into_iter()
        .cycle()
        .scan("AAA", |node, direction| {
            if *node == "ZZZ" {
                None
            } else {
                *node = match direction {
                    Direction::Left => map.get(node).unwrap().0,
                    Direction::Right => map.get(node).unwrap().1,
                };
                Some(())
            }
        })
        .count()
        .to_string()
}

fn part2(input: &str) -> String {
    let (directions, map) = parse_input(input).unwrap().1;

    let nodes = Vec::from_iter(map.keys().copied().filter(|n| n.as_bytes()[2] == b'A'));
    let iter = directions.into_iter().cycle();

    nodes
        .into_iter()
        .map(|start_node| {
            iter.clone()
                .scan(start_node, |node, direction| {
                    if node.as_bytes()[2] == b'Z' {
                        None
                    } else {
                        *node = match direction {
                            Direction::Left => map.get(node).unwrap().0,
                            Direction::Right => map.get(node).unwrap().1,
                        };
                        Some(())
                    }
                })
                .count()
        })
        .fold(1, |n1, n2| num::integer::lcm(n1, n2))
        .to_string()
}

fn parse_input(input: &str) -> nom::IResult<&str, (Vec<Direction>, BTreeMap<&str, (&str, &str)>)> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alphanumeric1, newline};
    use nom::combinator::map;
    use nom::combinator::value;
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;
    use nom::{multi::many1, sequence::delimited};

    separated_pair(
        many1(alt((
            value(Direction::Left, tag("L")),
            value(Direction::Right, tag("R")),
        ))),
        tag("\n\n"),
        map(
            separated_list1(
                newline,
                separated_pair(
                    alphanumeric1,
                    tag(" = "),
                    delimited(
                        tag("("),
                        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                        tag(")"),
                    ),
                ),
            ),
            |nodes| BTreeMap::from_iter(nodes.into_iter()),
        ),
    )(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
        let result = part1(input);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part1_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
        let result = part1(input);
        assert_eq!(result, "6");
    }

    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        let result = part2(input);
        assert_eq!(result, "6");
    }
}
