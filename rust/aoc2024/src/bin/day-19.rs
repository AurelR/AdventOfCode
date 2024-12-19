use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("data/input/input19.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (colors, towels) = parse(input).unwrap().1;
    let mut re_str = String::from("^(");
    re_str += &colors.join("|");
    re_str += ")*$";

    let re = Regex::new(&re_str).unwrap();
    let result = towels.into_iter().filter(|t| re.is_match(t)).count();
    result.to_string()
}

fn part2(input: &str) -> String {
    "".to_string()
}

fn parse(input: &str) -> nom::IResult<&str, (Vec<&str>, Vec<&str>)> {
    use nom::bytes::complete::is_a;
    use nom::bytes::complete::tag;
    use nom::character::complete::newline;
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;

    separated_pair(
        separated_list1(tag(", "), is_a("wubrg")),
        tag("\n\n"),
        separated_list1(newline, is_a("wubrg")),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

        let result = part1(input);
        assert_eq!("6", result);
    }

    #[test]
    fn test_part2() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

        let result = part2(input);
        assert_eq!("16", result);
    }
}
