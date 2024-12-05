use std::collections::BTreeMap;
use std::collections::BTreeSet;

type NumTy = i32;
type Set = BTreeSet<NumTy>;
type Map = BTreeMap<NumTy, Set>;

fn main() {
    let input = std::fs::read_to_string("data/input/input05.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (rules, updates) = parse(input).unwrap().1;

    let mut check = Map::new();
    for &(rl, rr) in &rules {
        check.entry(rl).or_default().insert(rr);
    }

    let mut result = 0;

    'outer: for update in &updates {
        let mut seen = Set::new();
        for n in update {
            if check.get(n).is_some() && seen.intersection(&check[n]).count() != 0 {
                continue 'outer;
            }
            seen.insert(*n);
        }
        result += update[update.len() / 2];
    }

    result.to_string()
}

fn part2(input: &str) -> String {
    let (rules, updates) = parse(input).unwrap().1;

    let mut check = Map::new();
    for &(rl, rr) in &rules {
        check.entry(rl).or_default().insert(rr);
    }

    let mut result = 0;
    for update in &updates {
        let mut sorted = update.clone();
        sorted.sort_by(|a, b| {
            if a == b {
                std::cmp::Ordering::Equal
            } else {
                match check.get(a) {
                    Some(greater) => {
                        if greater.contains(b) {
                            std::cmp::Ordering::Less
                        } else {
                            std::cmp::Ordering::Greater
                        }
                    }
                    None => std::cmp::Ordering::Greater,
                }
            }
        });
        if update != &sorted {
            result += sorted[sorted.len() / 2];
        }
    }
    result.to_string()
}

fn parse(input: &str) -> nom::IResult<&str, (Vec<(NumTy, NumTy)>, Vec<Vec<NumTy>>)> {
    use nom::bytes::complete::tag;
    use nom::character::complete::i32 as int32;
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;
    separated_pair(
        separated_list1(tag("\n"), separated_pair(int32, tag("|"), int32)),
        tag("\n\n"),
        separated_list1(tag("\n"), separated_list1(tag(","), int32)),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

        let result = part1(input);
        assert_eq!("143", result);
    }

    #[test]
    fn test_part2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

        let result = part2(input);
        assert_eq!("123", result);
    }
}
