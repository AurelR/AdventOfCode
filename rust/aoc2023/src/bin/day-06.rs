type NumTy = i64;
use nom::character::complete::i64 as num_parser;

fn main() {
    let input = std::fs::read_to_string("data/input/input06.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let data = parse_input(input)
        .map(|(_s, (time, dist))| (time.into_iter().zip(dist).collect::<Vec<_>>()))
        .unwrap();

    data.into_iter()
        .map(|(time, dist)| {
            (0..=time)
                .map(|t| (time - t) * t)
                .filter(|d| *d > dist)
                .count()
        })
        .product::<usize>()
        .to_string()
}

fn part2(input: &str) -> String {
    let (time, dist) = parse_input(input)
        .map(|(_s, (time, dist))| (vec_to_num(time), vec_to_num(dist)))
        .unwrap();

    (0..=time)
        .map(|t| (time - t) * t)
        .filter(|d| *d > dist)
        .count()
        .to_string()
}

fn vec_to_num(v: Vec<NumTy>) -> NumTy {
    v.iter().map(NumTy::to_string).collect::<String>().parse().unwrap()
}

fn parse_input(input: &str) -> nom::IResult<&str, (Vec<NumTy>, Vec<NumTy>)> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{newline, space1};
    use nom::multi::many1;
    use nom::sequence::{preceded, separated_pair};

    separated_pair(
        preceded(tag("Time:"), many1(preceded(space1, num_parser))),
        newline,
        preceded(tag("Distance:"), many1(preceded(space1, num_parser))),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        let result = part1(input);
        assert_eq!(result, "288");
    }

    #[test]
    fn test_part2() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        let result = part2(input);
        assert_eq!(result, "71503");
    }
}
