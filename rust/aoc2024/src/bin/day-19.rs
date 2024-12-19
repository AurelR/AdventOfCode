use std::collections::BTreeMap;

use regex::Regex;
type Cache<'a> = BTreeMap<&'a str, usize>;

fn main() {
    let input = std::fs::read_to_string("data/input/input19.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (colors, towels) = parse(input).unwrap().1;
    let re = Regex::new(&format!("^({})*$", colors.join("|"))).unwrap();
    let result = towels.into_iter().filter(|t| re.is_match(t)).count();
    result.to_string()
}

fn part2(input: &str) -> String {
    let (colors, towels) = parse(input).unwrap().1;
    let mut cache = Cache::new();
    towels
        .into_iter()
        .map(|t| count_matches(t, &colors, &mut cache))
        .sum::<usize>()
        .to_string()
}

fn count_matches<'a, 'b>(t: &'a str, colors: &[&str], cache: &'b mut Cache<'a>) -> usize {
    if t == "" {
        1
    } else if let Some(&count) = cache.get(t) {
        count
    } else {
        colors
            .iter()
            .map(|c| {
                if t.starts_with(c) {
                    let rest_t = &t[c.len()..];
                    let count = count_matches(rest_t, colors, cache);
                    cache.insert(rest_t, count);
                    count
                } else {
                    0
                }
            })
            .sum()
    }
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
