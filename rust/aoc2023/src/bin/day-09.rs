type NumTy = i32;
use nom::character::complete::i32 as num_parser;

fn main() {
    let input = std::fs::read_to_string("data/input/input09.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let sequences = parse_input(input).unwrap().1;
    sequences
        .into_iter()
        .map(|sequence| {
            let mut lasts = Vec::new();
            let mut current = sequence;
            while current.iter().any(|el| *el != 0) {
                lasts.push(*current.last().unwrap());
                current = current.windows(2).map(|pair| pair[1] - pair[0]).collect();
            }
            lasts.into_iter().sum::<NumTy>()
        })
        .sum::<NumTy>()
        .to_string()
}

fn part2(input: &str) -> String {
    let sequences = parse_input(input).unwrap().1;
    sequences
        .into_iter()
        .map(|sequence| {
            let mut firsts = Vec::new();
            let mut current = sequence;
            while current.iter().any(|el| *el != 0) {
                firsts.push(*current.first().unwrap());
                current = current.windows(2).map(|pair| pair[1] - pair[0]).collect();
            }
            firsts.into_iter().rev().reduce(|n1, n2| n2 - n1).unwrap()
        })
        .sum::<NumTy>()
        .to_string()
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<Vec<NumTy>>> {
    use nom::character::complete::{newline, space1};
    use nom::multi::separated_list1;

    separated_list1(newline, separated_list1(space1, num_parser))(input)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        let result = part1(input);
        assert_eq!(result, "114");
    }

    #[test]
    fn test_part2() {
        let input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        let result = part2(input);
        assert_eq!(result, "2");
    }
}
