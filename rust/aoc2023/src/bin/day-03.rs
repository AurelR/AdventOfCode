use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = std::fs::read_to_string("data/input/input03.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

type NumTy = i32;

fn part1(input: &str) -> String {
    let mut symbols = BTreeMap::new();
    let mut numbers = BTreeMap::new();

    let mut number = 0;
    let mut num_mode = false;
    let mut num_start = 0;

    for (row, line) in input.lines().enumerate() {
        let row = row as NumTy;
        for (col, c) in line.char_indices() {
            let col = col as NumTy;
            match c {
                '.' => {
                    if num_mode {
                        num_mode = false;
                        numbers.insert((row, num_start, col - 1), number);
                        number = 0;
                    }
                }
                d if '0' <= d && d <= '9' => {
                    if !num_mode {
                        num_mode = true;
                        num_start = col;

                    }
                    number = number * 10 + d.to_digit(10).expect("Digits expected");
                }
                s => {
                    symbols.insert((row, col), s);
                    if num_mode {
                        num_mode = false;
                        numbers.insert((row, num_start, col - 1), number);
                        number = 0;
                    }
                }
            }
        }
        if num_mode {
            num_mode = false;
            numbers.insert((row, num_start, line.as_bytes().len() as NumTy), number);
            number = 0;
        }
    }

    let mut sum = 0;
    for (&(row, c_begin, c_end) ,&num) in &numbers {
        let mut has_symbol = false;
        for r in row-1 ..= row+1 {
            for c in c_begin-1 ..= c_end+1 {
                if symbols.contains_key(&(r, c)) {
                    has_symbol = true;
                }
            }
        }
        if has_symbol {
            sum += num;
        }
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut symbols = BTreeMap::new();
    let mut numbers = BTreeMap::new();

    let mut number = 0;
    let mut num_mode = false;
    let mut num_start = 0;

    for (row, line) in input.lines().enumerate() {
        let row = row as NumTy;
        for (col, c) in line.char_indices() {
            let col = col as NumTy;
            match c {
                '.' => {
                    if num_mode {
                        num_mode = false;
                        for c in num_start .. col {
                            numbers.insert((row, c), number);
                        }
                        number = 0;
                    }
                }
                d if '0' <= d && d <= '9' => {
                    if !num_mode {
                        num_mode = true;
                        num_start = col;

                    }
                    number = number * 10 + d.to_digit(10).expect("Digits expected") as NumTy;
                }
                s => {
                    symbols.insert((row, col), s);
                    if num_mode {
                        num_mode = false;
                        for c in num_start .. col {
                            numbers.insert((row, c), number);
                        }
                        number = 0;
                    }
                }
            }
        }
        if num_mode {
            num_mode = false;
            for c in num_start ..= line.as_bytes().len() as NumTy {
                numbers.insert((row, c), number);
            }
            number = 0;
        }
    }

    let mut sum = 0;
    for (&(row, col) ,&sym) in &symbols {
        if sym != '*' {
            continue;
        }

        let mut gears = BTreeSet::new();
        for r in row-1 ..= row+1 {
            for c in col-1 ..= col+1 {
                if let Some(&num) = numbers.get(&(r, c)) {
                    gears.insert(num);
                }
            }
        }

        if gears.len() == 2 {
            sum += gears.into_iter().product::<NumTy>();
        }
    }

    sum.to_string()
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
