//use itertools::Itertools;
use regex::Regex;
type NumTy = i32;

fn main() {
    let input = std::fs::read_to_string("data/input/input03.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut result = 0;
    for l in input.lines() {
        for m in re.captures_iter(l) {
            result += m[1].parse::<NumTy>().unwrap() * m[2].parse::<NumTy>().unwrap()
        }
    }
    result.to_string()
}

fn part2(input: &str) -> String {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut result = 0;
    for l in input.lines() {
        for m in re.captures_iter(l) {
            if &m[0] == "do()" {
                enabled = true;
            } else if &m[0] == "don't()" {
                enabled = false;
            } else if enabled {
                result += m[1].parse::<NumTy>().unwrap() * m[2].parse::<NumTy>().unwrap()
            }
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

        let result = part1(input);
        assert_eq!("161", result);
    }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

        let result = part2(input);
        assert_eq!("48", result);
    }
}
