type NumTy = i32;
use nom::{character::complete::i32 as num_parser, sequence::preceded};
use std::collections::BTreeMap;

fn main() {
    let input = std::fs::read_to_string("data/input/input19.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (workflows, parts) = parse_input(input).unwrap().1;
    let workflows = BTreeMap::from_iter(workflows.into_iter().map(|w| (w.name, w)));

    let mut accepted_sum = 0;
    'part: for part in parts {
        let mut wn = "in";
        dbg!(part);
        'workflow: loop {
            dbg!(wn);
            let w = workflows.get(wn).unwrap();
            for r in &w.rules {
                match r {
                    Rule::Send(Target::Accpet) => {
                        accepted_sum += part.x + part.m + part.a + part.s;
                        continue 'part;
                    }
                    Rule::Send(Target::Reject) => continue 'part,
                    Rule::Send(Target::Workflow(w)) => {
                        wn = *w;
                        continue 'workflow;
                    }
                    Rule::SendGreater(c, cmp, t) => {
                        let val = match c {
                            Category::X => part.x,
                            Category::M => part.m,
                            Category::A => part.a,
                            Category::S => part.s,
                        };
                        if val > *cmp {
                            match t {
                                Target::Workflow(w) => {
                                    wn = *w;
                                    continue 'workflow;
                                }
                                Target::Accpet => {
                                    accepted_sum += part.x + part.m + part.a + part.s;
                                    continue 'part;
                                }
                                Target::Reject => continue 'part,
                            }
                        }
                    }
                    Rule::SendLess(c, cmp, t) => {
                        let val = match c {
                            Category::X => part.x,
                            Category::M => part.m,
                            Category::A => part.a,
                            Category::S => part.s,
                        };
                        if val < *cmp {
                            match t {
                                Target::Workflow(w) => {
                                    wn = *w;
                                    continue 'workflow;
                                }
                                Target::Accpet => {
                                    accepted_sum += part.x + part.m + part.a + part.s;
                                    continue 'part;
                                }
                                Target::Reject => continue 'part,
                            }
                        }
                    }
                }
            }
        }
    }
    accepted_sum.to_string()
}

fn part2(input: &str) -> String {
    let (workflows, parts) = parse_input(input).unwrap().1;
    let workflows = BTreeMap::from_iter(workflows.into_iter().map(|w| (w.name, w)));
    "".to_string()
}

fn parse_input(input: &str) -> nom::IResult<&str, (Vec<Workflow>, Vec<Part>)> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, char, newline};
    use nom::combinator::{map, value};
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{delimited, preceded, separated_pair, tuple};

    separated_pair(
        separated_list1(
            newline,
            map(
                tuple((
                    alpha1,
                    delimited(char('{'), separated_list1(char(','), parse_rule), char('}')),
                )),
                |(name, rules)| Workflow { name, rules },
            ),
        ),
        tag("\n\n"),
        separated_list1(
            newline,
            map(
                delimited(
                    tag("{x="),
                    tuple((
                        num_parser,
                        preceded(tag(",m="), num_parser),
                        preceded(tag(",a="), num_parser),
                        preceded(tag(",s="), num_parser),
                    )),
                    char('}'),
                ),
                |(x, m, a, s)| Part { x, m, a, s },
            ),
        ),
    )(input)
}

fn parse_rule(input: &str) -> nom::IResult<&str, Rule> {
    use nom::branch::alt;
    use nom::character::complete::{alpha1, char, one_of};
    use nom::combinator::{map, value};
    use nom::sequence::{delimited, preceded, separated_pair, tuple};
    alt((
        map(
            tuple((
                parse_category,
                one_of("<>"),
                num_parser,
                preceded(char(':'), parse_target),
            )),
            |(category, cmp, cmp_value, target)| match cmp {
                '>' => Rule::SendGreater(category, cmp_value, target),
                '<' => Rule::SendLess(category, cmp_value, target),
                _ => unreachable!(),
            },
        ),
        map(parse_target, |t| Rule::Send(t)),
    ))(input)
}

fn parse_category(input: &str) -> nom::IResult<&str, Category> {
    use nom::branch::alt;
    use nom::character::complete::{alpha1, char, one_of};
    use nom::combinator::{map, value};
    use nom::sequence::{delimited, preceded, separated_pair, tuple};
    map(one_of("xmas"), |c| match c {
        'x' => Category::X,
        'm' => Category::M,
        'a' => Category::A,
        's' => Category::S,
        _ => unreachable!(),
    })(input)
}

fn parse_target(input: &str) -> nom::IResult<&str, Target> {
    use nom::branch::alt;
    use nom::character::complete::{alpha1, char};
    use nom::combinator::{map, value};
    alt((
        value(Target::Accpet, char('A')),
        value(Target::Reject, char('R')),
        map(alpha1, |w| Target::Workflow(w)),
    ))(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rule<'a> {
    Send(Target<'a>),
    SendGreater(Category, NumTy, Target<'a>),
    SendLess(Category, NumTy, Target<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target<'a> {
    Accpet,
    Reject,
    Workflow(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Part {
    x: NumTy,
    m: NumTy,
    a: NumTy,
    s: NumTy,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";
        let result = part1(input);
        assert_eq!(result, "19114");
    }

    #[test]
    #[ignore = "not done yet"]
    fn test_part2() {
        let input = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";
        let result = part2(input);
        assert_eq!(result, "todo");
    }
}
