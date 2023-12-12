type NumTy = u64;
use nom::character::complete::u64 as num_parser;

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
    let _data = parse_input(input).unwrap().1;
    "".to_string()
}

#[allow(unused_imports)]
fn parse_input(input: &str) -> nom::IResult<&str, Vec<(Vec<State>, Vec<NumTy>)>> {
    use nom::branch::alt;
    use nom::bytes::complete::is_a;
    use nom::character::complete::{char, newline, space1};
    use nom::combinator::{map, opt, value};
    use nom::multi::{many1, many1_count, separated_list1};
    use nom::sequence::{delimited, separated_pair};

    // separated_list1(
    //     newline,
    //     separated_pair(
    //         delimited(
    //             opt(is_a(".")),
    //             separated_list1(
    //                 is_a("."),
    //                 many1(alt((
    //                     map(many1_count(char('#')), |c| (State::Broken, c as NumTy)),
    //                     map(many1_count(char('?')), |c| (State::Unknown, c as NumTy)),
    //                 ))),
    //             ),
    //             opt(is_a(".")),
    //         ),
    //         space1,
    //         separated_list1(char(','), num_parser),
    //     ),
    // )(input)

    // separated_list1(
    //     newline,
    //     separated_pair(
    //         delimited(
    //             opt(is_a(".")),
    //             separated_list1(
    //                 is_a("."),
    //                 many1(alt((
    //                     value(State::Broken, char('#')),
    //                     value(State::Unknown, char('?')),
    //                 ))),
    //             ),
    //             opt(is_a(".")),
    //         ),
    //         space1,
    //         separated_list1(char(','), num_parser),
    //     ),
    // )(input)

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
    let (zero_mask, one_mask) = states_to_masks(&states);
    // dbg!(counts);
    // println!("{zero_mask:016b}\n{one_mask:016b}\n");
    let total = counts.iter().sum::<NumTy>();
    let mut count = 0;
    for i in 0..(1 << states.len()) {
        if (i & one_mask == one_mask)
            && (!i & zero_mask == zero_mask)
            && i.count_ones() as NumTy == total
            && matches_counts(i, counts)
        {
            // println!("{i:016b}");
            count += 1;
        }
    }
    count
}

fn states_to_masks(states: &[State]) -> (NumTy, NumTy) {
    let mut working_mask = 0;
    let mut broken_mask = 0;
    for s in states.iter().rev() {
        working_mask <<= 1;
        broken_mask <<= 1;

        match s {
            State::Working => working_mask |= 1,
            State::Broken => broken_mask |= 1,
            State::Unknown => {}
        }
    }
    (working_mask, broken_mask)
}

fn matches_counts(mut arrangment: NumTy, counts: &[NumTy]) -> bool {
    for c in counts.iter() {
        arrangment >>= arrangment.trailing_zeros();
        if arrangment.trailing_ones() != *c as u32 {
            return false;
        }
        arrangment >>= arrangment.trailing_ones();
    }
    arrangment == 0
}

// fn count_arrangments_simple(states: &[(State, NumTy)], count: NumTy) -> NumTy {
//     let total = states.iter().map(|(_s, c)| c).sum::<NumTy>();
//     match total.cmp(&count) {
//         std::cmp::Ordering::Less => dbg!(0),
//         std::cmp::Ordering::Equal => 1,
//         std::cmp::Ordering::Greater => match  {

//         },
//     }
// }

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
    #[ignore]
    fn test_part2() {
        let input = "\
";
        let result = part2(input);
        assert_eq!(result, "todo");
    }
}
