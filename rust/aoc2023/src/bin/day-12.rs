type NumTy = u64;
use nom::character::complete::u64 as num_parser;
use std::collections::HashMap as Cache;

fn main() {
    let input = std::fs::read_to_string("data/input/input12.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let data = parse_input(input).unwrap().1;
    data.iter()
        .map(|(states, counts)| count_arrangments(states, counts))
        .sum::<NumTy>()
        .to_string()
}

fn part2(input: &str) -> String {
    let data = parse_input(input).unwrap().1;
    data.iter()
        .map(|(states, counts)| {
            let (unfolded_states, unfolded_counts) = unfold(states, counts);
            count_arrangments(&unfolded_states, &unfolded_counts)
        })
        .sum::<NumTy>()
        .to_string()
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<(Vec<State>, Vec<NumTy>)>> {
    use nom::branch::alt;
    use nom::character::complete::{char, newline, space1};
    use nom::combinator::value;
    use nom::multi::{many1, separated_list1};
    use nom::sequence::separated_pair;

    separated_list1(
        newline,
        separated_pair(
            many1(alt((
                value(State::Working, char('.')),
                value(State::Broken, char('#')),
                value(State::Unknown, char('?')),
            ))),
            space1,
            separated_list1(char(','), num_parser),
        ),
    )(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Working,
    Broken,
    Unknown,
}

fn count_arrangments(states: &[State], counts: &[NumTy]) -> NumTy {
    let mut cache = Cache::new();
    search(states, counts, None, &mut cache)
}

fn search(
    states: &[State],
    counts: &[NumTy],
    inside: Option<NumTy>,
    cache: &mut Cache<(usize, NumTy, usize), NumTy>,
) -> NumTy {
    let key = (states.len(), inside.unwrap_or(0), counts.len());
    if let Some(&c) = cache.get(&key) {
        return c;
    }

    if states.is_empty() {
        return match (inside, counts.len()) {
            (None, 0) => 1,
            (Some(c), 1) if c == counts[0] => 1,
            _ => 0,
        };
    }
    if counts.is_empty() && inside.is_some() {
        return 0;
    }

    let result = match (states[0], inside) {
        (State::Working, None) => search(&states[1..], counts, None, cache),
        (State::Working, Some(c)) if c != counts[0] => 0,
        (State::Working, Some(_)) => search(&states[1..], &counts[1..], None, cache),
        (State::Broken, None) => search(&states[1..], counts, Some(1), cache),
        (State::Broken, Some(c)) => search(&states[1..], counts, Some(c + 1), cache),
        (State::Unknown, Some(c)) => {
            let mut result = search(&states[1..], counts, Some(c + 1), cache);
            if c == counts[0] {
                result += search(&states[1..], &counts[1..], None, cache)
            }
            result
        }
        (State::Unknown, None) => {
            search(&states[1..], counts, Some(1), cache) + search(&states[1..], counts, None, cache)
        }
    };
    cache.insert(key, result);
    return result;
}

fn unfold(states: &[State], counts: &[NumTy]) -> (Vec<State>, Vec<NumTy>) {
    (
        [states, states, states, states, states].join(&State::Unknown),
        counts.repeat(5),
    )
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_count_arrangments_1() {
        let input = "???.### 1,1,3";
        let (state, counts) = &parse_input(input).unwrap().1[0];
        let result = count_arrangments(state, counts);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_count_arrangments_2() {
        let input = ".??..??...?##. 1,1,3";
        let (state, counts) = &parse_input(input).unwrap().1[0];
        let result = count_arrangments(state, counts);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_count_arrangments_3() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        let (state, counts) = &parse_input(input).unwrap().1[0];
        let result = count_arrangments(state, counts);
        assert_eq!(result, 1);
    }
    #[test]
    fn test_count_arrangments_4() {
        let input = "????.#...#... 4,1,1";
        let (state, counts) = &parse_input(input).unwrap().1[0];
        let result = count_arrangments(state, counts);
        assert_eq!(result, 1);
    }
    #[test]
    fn test_count_arrangments_5() {
        let input = "????.######..#####. 1,6,5";
        let (state, counts) = &parse_input(input).unwrap().1[0];
        let result = count_arrangments(state, counts);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_count_arrangments_6() {
        let input = "?###???????? 3,2,1";
        let (state, counts) = &parse_input(input).unwrap().1[0];
        let result = count_arrangments(state, counts);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_count_arrangments_7() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        let (states, counts) = &parse_input(input).unwrap().1[0];
        let (states_unfolded, counts_unfolded) = unfold(states, counts);
        let result = count_arrangments(&states_unfolded, &counts_unfolded);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_unfold1() {
        let input = &parse_input(".# 1").unwrap().1[0];
        let expected = &parse_input(".#?.#?.#?.#?.# 1,1,1,1,1").unwrap().1[0];
        let result = unfold(&input.0, &input.1);
        assert_eq!(expected, &result);
    }

    #[test]
    fn test_unfold2() {
        let input = &parse_input("???.### 1,1,3").unwrap().1[0];
        let expected =
            &parse_input("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3")
                .unwrap()
                .1[0];
        let result = unfold(&input.0, &input.1);
        assert_eq!(expected, &result);
    }

    #[test]
    fn test_unfold3() {
        let input = &parse_input("?#?#?#?#?#?#?#? 1,3,1,6").unwrap().1[0];
        let expected =
            &parse_input("?#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#? 1,3,1,6,1,3,1,6,1,3,1,6,1,3,1,6,1,3,1,6")
                .unwrap()
                .1[0];
        let result = unfold(&input.0, &input.1);
        assert_eq!(expected, &result);
    }

    #[test]
    fn test_part1() {
        let input = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let result = part1(input);
        assert_eq!(result, "21");
    }

    #[test]
    fn test_part2() {
        let input = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let result = part2(input);
        assert_eq!(result, "525152");
    }
}
