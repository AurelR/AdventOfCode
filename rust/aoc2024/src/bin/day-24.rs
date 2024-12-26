use std::collections::BTreeMap;

type NumTy = u64;
use nom::character::complete::u64 as parse_int;

fn main() {
    let input = std::fs::read_to_string("data/input/input24.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (mut values, mut ops) = parse(input).unwrap().1;
    while ops.len() != 0 {
        ops.retain(|(l, op, r, t)| match (values.get(l), values.get(r)) {
            (None, None) => true,
            (None, Some(_)) => true,
            (Some(_), None) => true,
            (Some(vl), Some(vr)) => {
                values.insert(*t, op.eval(*vl, *vr));
                false
            }
        });
    }
    values
        .into_iter()
        .filter(|(k, _v)| k.starts_with("z"))
        .rev()
        .fold(0, |c, n| c << 1 | n.1)
        .to_string()
}

fn part2(input: &str) -> String {
    let (mut values, mut ops) = parse(input).unwrap().1;

    // manually found them by inspecting graph with graphviz
    ops = ops
        .into_iter()
        .map(|o| match o {
            ("jvj", Op::Xor, "mvs", "mkk") => ("jvj", Op::Xor, "mvs", "z10"),
            ("mvs", Op::And, "jvj", "z10") => ("mvs", Op::And, "jvj", "mkk"),

            ("vjh", Op::Or, "fhq", "z14") => ("vjh", Op::Or, "fhq", "qbw"),
            ("ndm", Op::Xor, "tsp", "qbw") => ("ndm", Op::Xor, "tsp", "z14"),

            ("y34", Op::And, "x34", "z34") => ("y34", Op::And, "x34", "wcb"),
            ("jmq", Op::Xor, "mdh", "wcb") => ("jmq", Op::Xor, "mdh", "z34"),

            ("x25", Op::Xor, "y25", "wjb") => ("x25", Op::Xor, "y25", "cvp"),
            ("x25", Op::And, "y25", "cvp") => ("x25", Op::And, "y25", "wjb"),
            _ => o,
        })
        .collect();

    // println!("digraph Adder {{");
    // for o in &ops {
    //     let op = o.1.to_str();
    //     println!("{0} -> \"{0} {2} {1}\"", o.0, o.2, op);
    //     println!("{1} -> \"{0} {2} {1}\"", o.0, o.2, op);
    //     println!("\"{0} {2} {1}\" -> {3}", o.0, o.2, op, o.3);
    // }
    // println!("}}");
    while ops.len() != 0 {
        ops.retain(|(l, op, r, t)| match (values.get(l), values.get(r)) {
            (None, None) => true,
            (None, Some(_)) => true,
            (Some(_), None) => true,
            (Some(vl), Some(vr)) => {
                values.insert(*t, op.eval(*vl, *vr));
                false
            }
        });
    }
    let x = values
        .iter()
        .filter(|(k, _v)| k.starts_with("x"))
        .rev()
        .fold(0, |c, n| c << 1 | n.1);

    let y = values
        .iter()
        .filter(|(k, _v)| k.starts_with("y"))
        .rev()
        .fold(0, |c, n| c << 1 | n.1);

    let z = values
        .into_iter()
        .filter(|(k, _v)| k.starts_with("z"))
        .rev()
        .fold(0, |c, n| c << 1 | n.1);
    // eprintln!("{x:>46b}");
    // eprintln!("{y:>46b}");
    // eprintln!("{z:>46b}");
    // eprintln!("{:>46b}",x+y);
    // eprintln!("{:>46b}", z ^ (x + y));
    assert_eq!(z, x + y);
    let mut result = vec!["mkk", "z10", "z14", "qbw", "z34", "wcb", "wjb", "cvp"];
    result.sort();
    result.join(",")
}

fn parse(input: &str) -> nom::IResult<&str, (BTreeMap<&str, NumTy>, Vec<(&str, Op, &str, &str)>)> {
    use nom::Parser;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alphanumeric1, newline},
        multi::separated_list1,
        sequence::{delimited, preceded, separated_pair, tuple},
    };

    separated_pair(
        separated_list1(newline, separated_pair(alphanumeric1, tag(": "), parse_int))
            .map(|d| BTreeMap::from_iter(d)),
        tag("\n\n"),
        separated_list1(
            newline,
            tuple((
                alphanumeric1,
                delimited(
                    tag(" "),
                    alt((
                        tag("AND").map(|_| Op::And),
                        tag("OR").map(|_| Op::Or),
                        tag("XOR").map(|_| Op::Xor),
                    )),
                    tag(" "),
                ),
                alphanumeric1,
                preceded(tag(" -> "), alphanumeric1),
            )),
        ),
    )(input)
}

#[derive(Debug, Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn eval(&self, right: NumTy, left: NumTy) -> NumTy {
        match self {
            Op::And => right & left,
            Op::Or => right | left,
            Op::Xor => right ^ left,
        }
    }

    #[allow(dead_code)]
    fn to_str(&self) -> &'static str {
        match self {
            Op::And => "AND",
            Op::Or => "OR",
            Op::Xor => "XOR",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_a() {
        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

        let result: String = part1(input);
        assert_eq!("4", result);
    }

    #[test]
    fn test_part1_b() {
        let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

        let result: String = part1(input);
        assert_eq!("2024", result);
    }

    #[test]
    #[ignore = "todo"]
    fn test_part2() {
        let input = "
";

        let result = part2(input);
        assert_eq!("00000", result);
    }
}
