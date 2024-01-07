use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};
type NumTy = i32;

fn main() {
    let input = std::fs::read_to_string("data/input/input03.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let mut symbols = BTreeMap::new();
    let mut numbers = BTreeMap::new();

    parse_input(
        input,
        |pos, symbol| {
            symbols.insert(pos, symbol);
        },
        |pos, number| {
            numbers.insert(pos, number);
        },
    );

    numbers
        .into_iter()
        .filter(|((row, c_begin, c_end), _number)| {
            (row - 1..=row + 1)
                .cartesian_product(c_begin - 1..=c_end + 1)
                .any(|pos| symbols.contains_key(&pos))
        })
        .map(|(_pos, number)| number)
        .sum::<NumTy>()
        .to_string()
}

fn part2(input: &str) -> String {
    let mut symbols = BTreeMap::new();
    let mut numbers = BTreeMap::new();

    parse_input(
        input,
        |pos, symbol| {
            symbols.insert(pos, symbol);
        },
        |(row, col_start, col_end), number| {
            for c in col_start..=col_end {
                numbers.insert((row, c), number);
            }
        },
    );

    symbols
        .into_iter()
        .filter(|&(_pos, symbol)| symbol == '*')
        .map(|((row, col), _symbol)| {
            let gears = (row - 1..=row + 1)
                .cartesian_product(col - 1..=col + 1)
                .filter_map(|pos| numbers.get(&pos))
                .collect::<BTreeSet<_>>();
            if gears.len() == 2 {
                gears.into_iter().product::<NumTy>()
            } else {
                0
            }
        })
        .sum::<NumTy>()
        .to_string()
}

fn parse_input(
    input: &str,
    mut symbol_detected: impl FnMut((NumTy, NumTy), char),
    mut number_detected: impl FnMut((NumTy, NumTy, NumTy), NumTy),
) {
    for (row, line) in input.lines().enumerate() {
        let row = row as NumTy;
        let mut number = 0;
        let mut in_number = false;
        let mut num_start = 0;
        for (col, c) in line.char_indices() {
            let col = col as NumTy;
            let is_digit = match c {
                d if '0' <= d && d <= '9' => {
                    number = number * 10 + d.to_digit(10).expect("Digits expected") as NumTy;
                    true
                }
                '.' => false,
                s => {
                    symbol_detected((row, col), s);
                    false
                }
            };

            match (is_digit, in_number) {
                (true, false) => {
                    in_number = true;
                    num_start = col;
                }
                (false, true) => {
                    in_number = false;
                    number_detected((row, num_start, col - 1), number);
                    number = 0;
                }
                _ => {}
            }
        }
        if in_number {
            number_detected((row, num_start, line.as_bytes().len() as NumTy), number);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let result = part1(input);
        assert_eq!(result, "4361");
    }

    #[test]
    fn test_part2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let result = part2(input);
        assert_eq!(result, "467835");
    }
}
