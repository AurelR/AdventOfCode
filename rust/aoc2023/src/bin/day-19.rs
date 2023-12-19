type NumTy = i64;
use nom::character::complete::i64 as num_parser;
use std::collections::BTreeMap;
use std::ops::Range;

fn main() {
    let input = std::fs::read_to_string("data/input/input19.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (workflows, parts) = parse_input(input).unwrap().1;

    parts
        .into_iter()
        .map(|part| {
            let mut wn = "in";
            loop {
                let w = &workflows[wn];
                match w.process(&part) {
                    Target::Accept => {
                        return part.x + part.m + part.a + part.s;
                    }
                    Target::Reject => return 0,
                    Target::Workflow(w) => {
                        wn = w;
                    }
                }
            }
        })
        .sum::<NumTy>()
        .to_string()
}

fn part2(input: &str) -> String {
    let (workflows, _) = parse_input(input).unwrap().1;
    let mut accepted = Vec::new();
    let mut active_ranges = vec![(
        "in",
        PartRange {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        },
    )];

    let mut next_ranges = Vec::new();

    while !active_ranges.is_empty() {
        for (lw, mut part) in active_ranges {
            let w = &workflows[lw];
            for rule in &w.rules {
                let (matching, not_matching) = rule.condition.apply_range(part);
                if let Some(p) = matching {
                    match rule.target {
                        Target::Accept => accepted.push(p),
                        Target::Reject => {}
                        Target::Workflow(nw) => next_ranges.push((nw, p)),
                    }
                }

                if let Some(p) = not_matching {
                    part = p;
                } else {
                    break;
                }
            }
        }
        active_ranges = next_ranges;
        next_ranges = Vec::new();
    }
    accepted
        .into_iter()
        .map(|p| p.x.count() * p.m.count() * p.a.count() * p.s.count())
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &str) -> nom::IResult<&str, (BTreeMap<&str, Workflow>, Vec<Part>)> {
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, char, newline};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, preceded, separated_pair, tuple};

    map(
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
        ),
        |(workflows, parts)| {
            (
                BTreeMap::from_iter(workflows.into_iter().map(|w| (w.name, w))),
                parts,
            )
        },
    )(input)
}

fn parse_rule(input: &str) -> nom::IResult<&str, Rule> {
    use nom::branch::alt;
    use nom::character::complete::{char, one_of};
    use nom::combinator::map;
    use nom::sequence::{preceded, tuple};
    alt((
        map(
            tuple((
                parse_category,
                one_of("<>"),
                num_parser,
                preceded(char(':'), parse_target),
            )),
            |(category, cmp, cmp_value, target)| match cmp {
                '>' => Rule {
                    target,
                    condition: Condition::Greater(category, cmp_value),
                },
                '<' => Rule {
                    target,
                    condition: Condition::Less(category, cmp_value),
                },
                _ => unreachable!(),
            },
        ),
        map(parse_target, |target| Rule {
            target,
            condition: Condition::Always,
        }),
    ))(input)
}

fn parse_category(input: &str) -> nom::IResult<&str, Category> {
    use nom::character::complete::one_of;
    use nom::combinator::map;
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
        value(Target::Accept, char('A')),
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
struct Rule<'a> {
    target: Target<'a>,
    condition: Condition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    Always,
    Greater(Category, NumTy),
    Less(Category, NumTy),
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
    Accept,
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct PartRange {
    x: Range<NumTy>,
    m: Range<NumTy>,
    a: Range<NumTy>,
    s: Range<NumTy>,
}

impl<'a> Workflow<'a> {
    fn process(&self, part: &Part) -> Target<'a> {
        self.rules
            .iter()
            .find(|r| r.condition.apply(part))
            .expect("Last rule should always apply")
            .target
    }
}

impl Condition {
    fn apply(&self, part: &Part) -> bool {
        match self {
            Condition::Always => true,
            Condition::Greater(c, cmp) | Condition::Less(c, cmp) => {
                let val = match c {
                    Category::X => &part.x,
                    Category::M => &part.m,
                    Category::A => &part.a,
                    Category::S => &part.s,
                };
                if let Condition::Greater(_, _) = self {
                    val > cmp
                } else {
                    val < cmp
                }
            }
        }
    }
    fn apply_range(&self, part: PartRange) -> (Option<PartRange>, Option<PartRange>) {
        match self {
            Condition::Always => (Some(part), None),
            Condition::Less(category, limit) => {
                let val = match category {
                    Category::X => &part.x,
                    Category::M => &part.m,
                    Category::A => &part.a,
                    Category::S => &part.s,
                };
                if val.end <= *limit {
                    (Some(part), None)
                } else if val.start >= *limit {
                    (None, Some(part))
                } else {
                    let matching = val.start..*limit;
                    let not_matching = *limit..val.end;
                    match category {
                        Category::X => (
                            Some(PartRange {
                                x: matching,
                                ..part.clone()
                            }),
                            Some(PartRange {
                                x: not_matching,
                                ..part
                            }),
                        ),
                        Category::M => (
                            Some(PartRange {
                                m: matching,
                                ..part.clone()
                            }),
                            Some(PartRange {
                                m: not_matching,
                                ..part
                            }),
                        ),
                        Category::A => (
                            Some(PartRange {
                                a: matching,
                                ..part.clone()
                            }),
                            Some(PartRange {
                                a: not_matching,
                                ..part
                            }),
                        ),
                        Category::S => (
                            Some(PartRange {
                                s: matching,
                                ..part.clone()
                            }),
                            Some(PartRange {
                                s: not_matching,
                                ..part
                            }),
                        ),
                    }
                }
            }
            Condition::Greater(category, limit) => {
                let val = match category {
                    Category::X => &part.x,
                    Category::M => &part.m,
                    Category::A => &part.a,
                    Category::S => &part.s,
                };
                if *limit <= val.start {
                    (Some(part), None)
                } else if val.end <= *limit + 1 {
                    (None, Some(part))
                } else {
                    let matching = *limit + 1..val.end;
                    let not_matching = val.start..*limit + 1;
                    match category {
                        Category::X => (
                            Some(PartRange {
                                x: matching,
                                ..part.clone()
                            }),
                            Some(PartRange {
                                x: not_matching,
                                ..part
                            }),
                        ),
                        Category::M => (
                            Some(PartRange {
                                m: matching,
                                ..part.clone()
                            }),
                            Some(PartRange {
                                m: not_matching,
                                ..part
                            }),
                        ),
                        Category::A => (
                            Some(PartRange {
                                a: matching,
                                ..part.clone()
                            }),
                            Some(PartRange {
                                a: not_matching,
                                ..part
                            }),
                        ),
                        Category::S => (
                            Some(PartRange {
                                s: matching,
                                ..part.clone()
                            }),
                            Some(PartRange {
                                s: not_matching,
                                ..part
                            }),
                        ),
                    }
                }
            }
        }
    }
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
        assert_eq!(result, "167409079868000");
    }

    #[test]
    fn test_apply_range() {
        let part = PartRange {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        };
        let condition = Condition::Always;
        assert_eq!(
            (Some(part.clone()), None),
            condition.apply_range(part.clone())
        );

        let condition = Condition::Greater(Category::M, 2090);
        assert_eq!(
            (
                Some(PartRange {
                    x: 1..4001,
                    m: 2091..4001,
                    a: 1..4001,
                    s: 1..4001,
                }),
                Some(PartRange {
                    x: 1..4001,
                    m: 1..2091,
                    a: 1..4001,
                    s: 1..4001,
                })
            ),
            condition.apply_range(part.clone())
        );

        let condition = Condition::Greater(Category::X, 4000);
        assert_eq!(
            (None, Some(part.clone())),
            condition.apply_range(part.clone())
        );

        let condition = Condition::Greater(Category::S, 1);
        assert_eq!(
            (Some(part.clone()), None),
            condition.apply_range(part.clone())
        );

        let condition = Condition::Less(Category::A, 4001);
        assert_eq!(
            (Some(part.clone()), None),
            condition.apply_range(part.clone())
        );

        let condition = Condition::Less(Category::A, 1);
        assert_eq!(
            (None, Some(part.clone())),
            condition.apply_range(part.clone())
        );

        let condition = Condition::Less(Category::M, 2090);
        assert_eq!(
            (
                Some(PartRange {
                    x: 1..4001,
                    m: 1..2090,
                    a: 1..4001,
                    s: 1..4001,
                }),
                Some(PartRange {
                    x: 1..4001,
                    m: 2090..4001,
                    a: 1..4001,
                    s: 1..4001,
                })
            ),
            condition.apply_range(part.clone())
        );
    }
}
