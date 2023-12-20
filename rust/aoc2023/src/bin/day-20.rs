use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
type Modules<'a> = BTreeMap<&'a str, Module<'a>>;

fn main() {
    let input = std::fs::read_to_string("data/input/input20.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let data = parse_input(input).unwrap().1;
    let modules = reformat_data(data);

    let mut state = initalize_state(&modules);
    let mut pulses = [0i64; 2];
    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back(("broadcaster", false, ""));
        while let Some((cur_name, cur_signal, cur_tx)) = queue.pop_front() {
            pulses[cur_signal as usize] += 1;
            let Some(module) = modules.get(cur_name) else {
            continue;
        };
            match module.mtype {
                ModuleType::Broadcaster => {
                    for &rx in module.receiver.iter() {
                        queue.push_back((rx, cur_signal, cur_name))
                    }
                }
                ModuleType::FlipFlop => {
                    if !cur_signal {
                        if let State::FlipFlop(internal_state) = state.get_mut(cur_name).unwrap() {
                            *internal_state = !*internal_state;
                            for &rx in module.receiver.iter() {
                                queue.push_back((rx, *internal_state, cur_name))
                            }
                        }
                    }
                }
                ModuleType::Conjunction => {
                    if let State::Conjunction(internal_state) = state.get_mut(cur_name).unwrap() {
                        internal_state
                            .entry(cur_tx)
                            .and_modify(|el| *el = cur_signal);
                        let new_signal = !internal_state.iter().all(|(_k, v)| *v);
                        for &rx in module.receiver.iter() {
                            queue.push_back((rx, new_signal, cur_name))
                        }
                    }
                }
            }
        }
    }

    (pulses[0] * pulses[1]).to_string()
}

fn part2(input: &str) -> String {
    let _data = parse_input(input).unwrap().1;
    "".to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module<'a> {
    name: &'a str,
    mtype: ModuleType,
    receiver: Vec<&'a str>,
    transmitter: BTreeSet<&'a str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State<'a> {
    FlipFlop(bool),
    Conjunction(BTreeMap<&'a str, bool>),
    Other,
}

fn initalize_state<'a>(modules: &'a Modules) -> BTreeMap<&'a str, State<'a>> {
    BTreeMap::from_iter(modules.iter().map(|(&name, module)| {
        (
            name,
            match module.mtype {
                ModuleType::Broadcaster => State::Other,
                ModuleType::FlipFlop => State::FlipFlop(false),
                ModuleType::Conjunction => State::Conjunction(BTreeMap::from_iter(
                    module.transmitter.iter().map(|&tx| (tx, false)),
                )),
            },
        )
    }))
}

#[allow(unused_imports)]
fn parse_input(input: &str) -> nom::IResult<&str, Vec<Module>> {
    use nom::branch::alt;
    use nom::bytes::complete::{is_a, tag, take};
    use nom::character::complete::{alpha1, alphanumeric1, char, newline, space1};
    use nom::combinator::{map, opt, value};
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};

    separated_list1(
        newline,
        map(
            separated_pair(
                tuple((
                    map(opt(alt((tag("%"), tag("&")))), |prefix| match prefix {
                        Some("%") => ModuleType::FlipFlop,
                        Some("&") => ModuleType::Conjunction,
                        None => ModuleType::Broadcaster,
                        _ => unreachable!(),
                    }),
                    alpha1,
                )),
                tag(" -> "),
                separated_list1(tag(", "), alpha1),
            ),
            |((mtype, name), receiver)| Module {
                name,
                mtype,
                receiver,
                transmitter: Default::default(),
            },
        ),
    )(input)
}

fn reformat_data(data: Vec<Module>) -> Modules {
    let mut transmitter = BTreeMap::new();
    for m in &data {
        for &rx in &m.receiver {
            transmitter
                .entry(rx)
                .or_insert(BTreeSet::new())
                .insert(m.name);
        }
    }
    BTreeMap::from_iter(data.into_iter().map(|mut module| {
        if transmitter.contains_key(module.name) {
            module.transmitter = transmitter.remove(module.name).unwrap();
        }
        (module.name, module)
    }))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1a() {
        let input = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";
        let result = part1(input);
        assert_eq!(result, "32000000");
    }

    #[test]
    fn test_part1b() {
        let input = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";
        let result = part1(input);
        assert_eq!(result, "11687500");
    }

    #[test]
    #[ignore = "not done yet"]
    fn test_part2() {
        let input = "\
";
        let result = part2(input);
        assert_eq!(result, "todo");
    }
}
