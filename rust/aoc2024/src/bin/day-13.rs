type NumTy = i64;
type Pair = (NumTy, NumTy);
use nom::character::complete::i64 as int;

fn main() {
    let input = std::fs::read_to_string("data/input/input13.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let data = parse(input).unwrap().1;
    let mut result = 0;
    for (a, b, t) in data {
        let det_a = a.0 * b.1 - b.0 * a.1;
        let det_a_a = t.0 * b.1 - b.0 * t.1;
        let det_a_b = a.0 * t.1 - t.0 * a.1;
        let r_a = det_a_a / det_a;
        let r_b = det_a_b / det_a;
        if det_a_a % det_a == 0 && det_a_a % det_a == 0 {
            result += 3 * r_a + r_b;
        }
    }
    result.to_string()
}

fn part2(input: &str) -> String {
    let data = parse(input).unwrap().1;
    let mut result = 0;
    for (a, b, t) in data {
        let t = (t.0 + 10000000000000, t.1 + 10000000000000);
        let det_a = a.0 * b.1 - b.0 * a.1;
        let det_a_a = t.0 * b.1 - b.0 * t.1;
        let det_a_b = a.0 * t.1 - t.0 * a.1;
        let r_a = det_a_a / det_a;
        let r_b = det_a_b / det_a;
        if det_a_a % det_a == 0 && det_a_a % det_a == 0 {
            result += 3 * r_a + r_b;
        }
    }
    result.to_string()
}

fn parse(input: &str) -> nom::IResult<&str, Vec<(Pair, Pair, Pair)>> {
    use nom::bytes::complete::tag;
    use nom::multi::separated_list1;
    use nom::sequence::{separated_pair, tuple};
    use nom::{character::complete::newline, sequence::delimited};

    separated_list1(
        newline,
        tuple((
            delimited(
                tag("Button A: X+"),
                separated_pair(int, tag(", Y+"), int),
                newline,
            ),
            delimited(
                tag("Button B: X+"),
                separated_pair(int, tag(", Y+"), int),
                newline,
            ),
            delimited(
                tag("Prize: X="),
                separated_pair(int, tag(", Y="), int),
                newline,
            ),
        )),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

        let result = part1(input);
        assert_eq!("480", result);
    }

    #[test]
    fn test_part2() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

        let result = part2(input);
        assert_eq!("875318608908", result);
    }
}
