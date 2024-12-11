use itertools::Itertools;
use nom::character::complete::newline;
use std::iter::repeat_n;

type NumTy = i64;
use nom::character::complete::i64 as int;

fn main() {
    let input = std::fs::read_to_string("data/input/input07.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let data = parse(input).unwrap().1;
    let mut result = 0;
    'outer: for (target, values) in data {
        for p in
            repeat_n([Operator::Plus, Operator::Mul], values.len() - 1).multi_cartesian_product()
        {
            let eval = evlatuate(&p, &values);
            if eval == target {
                result += target;
                continue 'outer;
            }
        }
    }
    result.to_string()
}

fn part2(input: &str) -> String {
    let data = parse(input).unwrap().1;
    let mut result = 0;
    'outer: for (target, values) in data {
        for p in repeat_n(
            [Operator::Plus, Operator::Mul, Operator::Concat],
            values.len() - 1,
        )
        .multi_cartesian_product()
        {
            let eval = evlatuate(&p, &values);
            if eval == target {
                result += target;
                continue 'outer;
            }
        }
    }
    result.to_string()
}

fn parse(input: &str) -> nom::IResult<&str, Vec<(NumTy, Vec<NumTy>)>> {
    use nom::bytes::complete::tag;
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;
    separated_list1(
        newline,
        separated_pair(int, tag(": "), separated_list1(tag(" "), int)),
    )(input)
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Plus,
    Mul,
    Concat,
}

fn evlatuate(ops: &[Operator], values: &[NumTy]) -> NumTy {
    let mut it = values.iter();
    let mut result = *it.next().unwrap();
    for (op, v) in ops.iter().zip_eq(it) {
        match op {
            Operator::Plus => result += v,
            Operator::Mul => result *= v,
            Operator::Concat => result = (result.to_string() + &v.to_string()).parse().unwrap(),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

        let result = part1(input);
        assert_eq!("3749", result);
    }

    #[test]
    fn test_part2() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

        let result = part2(input);
        assert_eq!("11387", result);
    }

    #[test]
    fn test_evaluate() {
        let result = evlatuate(&[], &[23]);
        assert_eq!(23, result);
    }
}
