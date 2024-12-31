use itertools::Itertools;
use pathfinding::directed::astar;
use std::{collections::HashMap, fmt::Display};

type Sequence = Vec<DPad>;
type Sequences = Vec<Sequence>;
type NumPadCache = HashMap<(NumPad, NumPad), Sequences>;

fn main() {
    let input = std::fs::read_to_string("data/input/input21.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let data = parse(input).unwrap().1;
    let mut result = 0;
    for code in data {
        let len = find_shortest_sequences(&code).first().unwrap().len();
        let num = code_to_number(&code);
        result += len * num;
    }
    result.to_string()
}

fn part2(_input: &str) -> String {
    "".to_string()
}

fn find_shortest_sequences(code: &[NumPad]) -> Sequences {
    let mut numpad_cache = NumPadCache::new();
    let subresult1 = get_numpad_sequences(code, &mut numpad_cache);
    let subresult2 = find_min_dpad(subresult1);
    let subresult3 = find_min_dpad(subresult2);
    subresult3
}

fn find_min_dpad(input: Sequences) -> Sequences {
    let mut len = usize::max_value();
    let mut subresult = Vec::new();
    for s in input {
        for sn in get_dpad_sequences(&s) {
            if sn.len() == len {
                subresult.push(sn);
            } else if sn.len() < len {
                len = sn.len();
                subresult.clear();
                subresult.push(sn);
            }
        }
    }
    subresult
}

fn get_numpad_sequences(code: &[NumPad], cache: &mut NumPadCache) -> Vec<Vec<DPad>> {
    let mut results = Sequences::new();
    let mut current = NumPad::Activate;
    for &next in code {
        results = extend_sequences(&results, &get_numpad_sequence(current, next, cache));
        current = next
    }
    results
}

fn get_numpad_sequence(from: NumPad, to: NumPad, cache: &mut NumPadCache) -> Sequences {
    if let Some(sequences) = cache.get(&(from, to)) {
        return sequences.clone();
    }

    let mut result = Sequences::new();
    if from == to {
        result.push(vec![DPad::Activate]);
    } else {
        let paths = astar::astar_bag(
            &from.to_pos(),
            |&p| {
                [(0, 1), (1, 0), (0, -1), (-1, 0)]
                    .iter()
                    .map(|&d| (p.0 + d.0, p.1 + d.1))
                    .filter(|pn| {
                        *pn != (0, 3) && (0..=2).contains(&pn.0) && (0..=3).contains(&pn.1)
                    })
                    .map(|pn| (pn, 1))
                    .collect::<Vec<_>>()
            },
            |&p| p.0.abs_diff(to.to_pos().0) + p.1.abs_diff(to.to_pos().1),
            |&p| p == to.to_pos(),
        )
        .unwrap();
        for path in paths.0 {
            let mut subresult = Vec::new();
            for (n1, n2) in path.into_iter().tuple_windows() {
                use std::cmp::Ordering::*;
                let dpad = match (n1.0.cmp(&n2.0), n1.1.cmp(&n2.1)) {
                    (Equal, Less) => DPad::Down,
                    (Equal, Greater) => DPad::Up,
                    (Less, Equal) => DPad::Right,
                    (Greater, Equal) => DPad::Left,
                    _ => unreachable!("Should not happen"),
                };
                subresult.push(dpad);
            }
            subresult.push(DPad::Activate);
            result.push(subresult);
        }
    }
    cache.insert((from, to), result.clone());
    result
}

fn extend_sequences(sequences: &Sequences, sequence: &Sequences) -> Sequences {
    if sequences.is_empty() {
        return sequence.clone();
    }

    sequences
        .into_iter()
        .cartesian_product(sequence)
        .map(|(sa, sb)| {
            let mut r = sa.clone();
            r.extend_from_slice(sb);
            r
        })
        .collect()
}

fn get_dpad_sequences(input: &[DPad]) -> Sequences {
    let mut result = Sequences::new();
    let mut current = DPad::Activate;
    for &next in input {
        let moves = match (current, next) {
            (DPad::Left, DPad::Left) => vec![vec![DPad::Activate]],
            (DPad::Left, DPad::Right) => vec![vec![DPad::Right, DPad::Right, DPad::Activate]],
            (DPad::Left, DPad::Activate) => vec![
                vec![DPad::Right, DPad::Right, DPad::Up, DPad::Activate],
                vec![DPad::Right, DPad::Up, DPad::Right, DPad::Activate],
            ],
            (DPad::Left, DPad::Up) => vec![vec![DPad::Right, DPad::Up, DPad::Activate]],
            (DPad::Left, DPad::Down) => vec![vec![DPad::Right, DPad::Activate]],
            (DPad::Right, DPad::Left) => vec![vec![DPad::Left, DPad::Left, DPad::Activate]],
            (DPad::Right, DPad::Right) => vec![vec![DPad::Activate]],
            (DPad::Right, DPad::Activate) => vec![vec![DPad::Up, DPad::Activate]],
            (DPad::Right, DPad::Up) => vec![
                vec![DPad::Left, DPad::Up, DPad::Activate],
                vec![DPad::Up, DPad::Left, DPad::Activate],
            ],
            (DPad::Right, DPad::Down) => vec![vec![DPad::Left, DPad::Activate]],
            (DPad::Activate, DPad::Left) => vec![
                vec![DPad::Left, DPad::Down, DPad::Left, DPad::Activate],
                vec![DPad::Down, DPad::Left, DPad::Left, DPad::Activate],
            ],
            (DPad::Activate, DPad::Right) => vec![vec![DPad::Down, DPad::Activate]],
            (DPad::Activate, DPad::Activate) => vec![vec![DPad::Activate]],
            (DPad::Activate, DPad::Up) => vec![vec![DPad::Left, DPad::Activate]],
            (DPad::Activate, DPad::Down) => vec![
                vec![DPad::Left, DPad::Down, DPad::Activate],
                vec![DPad::Down, DPad::Left, DPad::Activate],
            ],
            (DPad::Up, DPad::Left) => vec![vec![DPad::Down, DPad::Left, DPad::Activate]],
            (DPad::Up, DPad::Right) => vec![
                vec![DPad::Right, DPad::Down, DPad::Activate],
                vec![DPad::Down, DPad::Right, DPad::Activate],
            ],
            (DPad::Up, DPad::Activate) => vec![vec![DPad::Right, DPad::Activate]],
            (DPad::Up, DPad::Up) => vec![vec![DPad::Activate]],
            (DPad::Up, DPad::Down) => vec![vec![DPad::Down, DPad::Activate]],
            (DPad::Down, DPad::Left) => vec![vec![DPad::Left, DPad::Activate]],
            (DPad::Down, DPad::Right) => vec![vec![DPad::Right, DPad::Activate]],
            (DPad::Down, DPad::Activate) => vec![
                vec![DPad::Right, DPad::Up, DPad::Activate],
                vec![DPad::Up, DPad::Right, DPad::Activate],
            ],
            (DPad::Down, DPad::Up) => vec![vec![DPad::Up, DPad::Activate]],
            (DPad::Down, DPad::Down) => vec![vec![DPad::Activate]],
        };
        current = next;
        result = extend_sequences(&result, &moves)
    }
    result
}

fn code_to_number(code: &[NumPad]) -> usize {
    code[0].to_digit() * 100 + code[1].to_digit() * 10 + code[2].to_digit()
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Vec<NumPad>>> {
    use nom::character::complete::newline;
    use nom::character::complete::one_of;
    use nom::multi::{many1, separated_list1};
    use nom::Parser;

    separated_list1(
        newline,
        many1(one_of("0123456789A").map(|c| match c {
            '0' => NumPad::N0,
            '1' => NumPad::N1,
            '2' => NumPad::N2,
            '3' => NumPad::N3,
            '4' => NumPad::N4,
            '5' => NumPad::N5,
            '6' => NumPad::N6,
            '7' => NumPad::N7,
            '8' => NumPad::N8,
            '9' => NumPad::N9,
            'A' => NumPad::Activate,
            _ => unreachable!("Invalid Input"),
        })),
    )(input)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum NumPad {
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    Activate,
}

impl NumPad {
    fn to_digit(&self) -> usize {
        match self {
            NumPad::N0 => 0,
            NumPad::N1 => 1,
            NumPad::N2 => 2,
            NumPad::N3 => 3,
            NumPad::N4 => 4,
            NumPad::N5 => 5,
            NumPad::N6 => 6,
            NumPad::N7 => 7,
            NumPad::N8 => 8,
            NumPad::N9 => 9,
            NumPad::Activate => panic!("Cannot convert to digit"),
        }
    }

    fn to_pos(&self) -> (i8, i8) {
        match self {
            NumPad::N0 => (1, 3),
            NumPad::N1 => (0, 2),
            NumPad::N2 => (1, 2),
            NumPad::N3 => (2, 2),
            NumPad::N4 => (0, 1),
            NumPad::N5 => (1, 1),
            NumPad::N6 => (2, 1),
            NumPad::N7 => (0, 0),
            NumPad::N8 => (1, 0),
            NumPad::N9 => (2, 0),
            NumPad::Activate => (2, 3),
        }
    }
}

impl Display for NumPad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumPad::N0 => write!(f, "0"),
            NumPad::N1 => write!(f, "1"),
            NumPad::N2 => write!(f, "2"),
            NumPad::N3 => write!(f, "3"),
            NumPad::N4 => write!(f, "4"),
            NumPad::N5 => write!(f, "5"),
            NumPad::N6 => write!(f, "6"),
            NumPad::N7 => write!(f, "7"),
            NumPad::N8 => write!(f, "8"),
            NumPad::N9 => write!(f, "9"),
            NumPad::Activate => write!(f, "A"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum DPad {
    Left,
    Right,
    Activate,
    Up,
    Down,
}

impl DPad {
    fn from_char(c: char) -> Self {
        match c {
            '>' => DPad::Right,
            '<' => DPad::Left,
            '^' => DPad::Up,
            'v' => DPad::Down,
            'A' => DPad::Activate,
            _ => todo!("Invalid char"),
        }
    }
    fn to_pos(&self) -> (i8, i8) {
        match self {
            DPad::Right => (2, 1),
            DPad::Up => (1, 0),
            DPad::Left => (0, 1),
            DPad::Down => (1, 1),
            DPad::Activate => (2, 0),
        }
    }
}

impl Display for DPad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DPad::Right => write!(f, ">"),
            DPad::Up => write!(f, "^"),
            DPad::Left => write!(f, "<"),
            DPad::Down => write!(f, "v"),
            DPad::Activate => write!(f, "A"),
        }
    }
}

fn to_string(dpad: &[DPad]) -> String {
    dpad.iter().map(|d| format!("{d}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "029A
980A
179A
456A
379A
";

        let result: String = part1(input);
        assert_eq!("126384", result);
    }

    #[test]
    fn test_part1_sequences() {
        let input = vec![
            (
                [NumPad::N0, NumPad::N2, NumPad::N9, NumPad::Activate],
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
                    .chars()
                    .map(DPad::from_char)
                    .collect::<Vec<_>>(),
            ),
            (
                [NumPad::N9, NumPad::N8, NumPad::N0, NumPad::Activate],
                "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A"
                    .chars()
                    .map(DPad::from_char)
                    .collect::<Vec<_>>(),
            ),
            (
                [NumPad::N1, NumPad::N7, NumPad::N9, NumPad::Activate],
                "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
                    .chars()
                    .map(DPad::from_char)
                    .collect::<Vec<_>>(),
            ),
            (
                [NumPad::N4, NumPad::N5, NumPad::N6, NumPad::Activate],
                "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A"
                    .chars()
                    .map(DPad::from_char)
                    .collect::<Vec<_>>(),
            ),
            (
                [NumPad::N3, NumPad::N7, NumPad::N9, NumPad::Activate],
                "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
                    .chars()
                    .map(DPad::from_char)
                    .collect::<Vec<_>>(),
            ),
        ];

        for (code, expected_sequence) in input {
            assert!(find_shortest_sequences(&code).contains(&expected_sequence))
        }
    }

    #[test]
    fn test_part1_numpad_sequence() {
        let mut cache = NumPadCache::new();
        let mut expected = vec!["<A^A>^^AvvvA", "<A^A^>^AvvvA", "<A^A^^>AvvvA"];
        let mut results = get_numpad_sequences(
            &[NumPad::N0, NumPad::N2, NumPad::N9, NumPad::Activate],
            &mut cache,
        );
        expected.sort();
        results.sort();
        for (e, r) in std::iter::zip(expected, results) {
            assert_eq!(e, to_string(&r));
        }
    }

    #[test]
    fn test_part1_dpad_sequence1() {
        let input_str = "<A^A>^^AvvvA";
        let expected_str = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
        let input = input_str.chars().map(DPad::from_char).collect::<Vec<_>>();
        let expected = expected_str
            .chars()
            .map(DPad::from_char)
            .collect::<Vec<_>>();

        let result = get_dpad_sequences(&input);
        assert!(result.contains(&expected));
    }

    #[test]
    fn test_part1_dpad_sequence2() {
        let input_str = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
        let expected_str = "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A";
        let input = input_str.chars().map(DPad::from_char).collect::<Vec<_>>();
        let expected = expected_str
            .chars()
            .map(DPad::from_char)
            .collect::<Vec<_>>();

        let result = get_dpad_sequences(&input);
        assert!(result.contains(&expected));
    }

    #[test]
    #[ignore = "todo"]
    fn test_part2() {
        let input = "
";

        let result = part2(input);
        assert_eq!("0000000", result);
    }
}
