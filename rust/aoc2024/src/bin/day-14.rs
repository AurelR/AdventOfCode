type NumTy = i32;
type Pair = (NumTy, NumTy);
use nom::character::complete::i32 as int;

fn main() {
    let input = std::fs::read_to_string("data/input/input14.txt").unwrap();
    let result1 = part1(&input, 101, 103);
    println!("{}", result1);
    let result2 = part2(&input, 101, 103);
    println!("{}", result2);
}

fn part1(input: &str, maxx: NumTy, maxy: NumTy) -> String {
    let data = parse(input).unwrap().1;
    let mut result = Vec::new();
    for (p, v) in data {
        let value = (
            (p.0 + 100 * v.0).rem_euclid(maxx),
            (p.1 + 100 * v.1).rem_euclid(maxy),
        );
        result.push(value);
    }

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let q1x = 0..maxx / 2;
    let q1y = 0..maxy / 2;
    let q2x = (maxx / 2) + 1..maxx;
    let q2y = q1y.clone();
    let q3x = q1x.clone();
    let q3y = (maxy / 2) + 1..maxy;
    let q4x = q2x.clone();
    let q4y = q3y.clone();

    for p in result {
        if q1x.contains(&p.0) && q1y.contains(&p.1) {
            q1 += 1;
        }
        if q2x.contains(&p.0) && q2y.contains(&p.1) {
            q2 += 1;
        }
        if q3x.contains(&p.0) && q3y.contains(&p.1) {
            q3 += 1;
        }
        if q4x.contains(&p.0) && q4y.contains(&p.1) {
            q4 += 1;
        }
    }

    (q1 * q2 * q3 * q4).to_string()
}

fn part2(input: &str, maxx: NumTy, maxy: NumTy) -> String {
    let data = parse(input).unwrap().1;
    let mut result = 0;
    result.to_string()
}

fn parse(input: &str) -> nom::IResult<&str, Vec<(Pair, Pair)>> {
    use nom::bytes::complete::tag;
    use nom::multi::separated_list1;
    use nom::sequence::{separated_pair, tuple};
    use nom::{character::complete::newline, sequence::delimited, sequence::preceded};

    separated_list1(
        newline,
        separated_pair(
            preceded(tag("p="), separated_pair(int, tag(","), int)),
            tag(" "),
            preceded(tag("v="), separated_pair(int, tag(","), int)),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

        let result = part1(input, 11, 7);
        assert_eq!("12", result);
    }

    #[test]
    fn test_part2() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

        let result = part2(input, 11, 7);
        assert_eq!("000000", result);
    }
}
