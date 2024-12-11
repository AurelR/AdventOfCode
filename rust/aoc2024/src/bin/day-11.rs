use std::collections::BTreeMap;

type NumTy = i64;

type Cache = BTreeMap<(NumTy, NumTy), NumTy>;

fn main() {
    let input = std::fs::read_to_string("data/input/input11.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let mut data = parse(input);
    for _ in 0..25 {
        data = blink(data);
    }
    data.len().to_string()
}

fn part2(input: &str) -> String {
    let data = parse(input);
    let mut result = 0;
    let mut cache = Cache::new();
    for n in data {
        result += find_expansion(n, 75, &mut cache);
    }
    result.to_string()
}

fn parse(input: &str) -> Vec<NumTy> {
    input
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn blink(input: Vec<NumTy>) -> Vec<NumTy> {
    let mut output = Vec::new();
    for stone in input {
        let digits = count_digits(stone);
        if stone == 0 {
            output.push(1);
        } else if digits % 2 == 0 {
            let cut = 10i64.pow((digits / 2) as u32);
            output.push(stone / cut);
            output.push(stone % cut);
        } else {
            output.push(stone * 2024);
        }
    }
    output
}

fn count_digits(mut num: NumTy) -> NumTy {
    if num == 0 {
        return 1;
    }
    let mut count = 0;
    while num != 0 {
        num = num / 10;
        count += 1;
    }
    count
}

fn find_expansion(num: NumTy, its: NumTy, cache: &mut Cache) -> NumTy {
    if its == 0 {
        return 1;
    }
    if let Some(r) = cache.get(&(num, its)) {
        return *r;
    }

    let mut result = 0;
    for d in blink(vec![num]) {
        result += find_expansion(d, its - 1, cache)
    }
    cache.insert((num, its), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "125 17
";

        let result = part1(input);
        assert_eq!("55312", result);
    }

    #[test]
    fn test_part2() {
        let input = "125 17
";

        let result = part2(input);
        assert_eq!("65601038650482", result);
    }

    #[test]
    fn test_count_digits_1() {
        assert_eq!(1, count_digits(0));
    }

    #[test]
    fn test_count_digits_2() {
        assert_eq!(1, count_digits(3));
    }

    #[test]
    fn test_count_digits_3() {
        assert_eq!(2, count_digits(10));
    }

    #[test]
    fn test_count_digits_4() {
        assert_eq!(4, count_digits(4711));
    }
}
