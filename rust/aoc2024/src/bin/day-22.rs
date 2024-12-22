type NumTy = u64;

fn main() {
    let input = std::fs::read_to_string("data/input/input22.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let numbers = parse(input);
    numbers
        .into_iter()
        .map(|mut num| {
            for _ in 0..2000 {
                num = next_number(num);
            }
            num
        })
        .sum::<NumTy>()
        .to_string()
}

fn part2(_input: &str) -> String {
    "".to_string()
}

fn parse(input: &str) -> Vec<NumTy> {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>()
}

fn next_number(mut number: NumTy) -> NumTy {
    number = (number ^ (number * 64)) % 16777216;
    number = (number ^ (number / 32)) % 16777216;
    number = (number ^ (number * 2048)) % 16777216;
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1
10
100
2024
";

        let result: String = part1(input);
        assert_eq!("37327623", result);
    }

    #[test]
    fn test_next_number() {
        let mut number = 123;

        number = next_number(number);
        assert_eq!(15887950, number);

        number = next_number(number);
        assert_eq!(16495136, number);

        number = next_number(number);
        assert_eq!(527345, number);

        number = next_number(number);
        assert_eq!(704524, number);

        number = next_number(number);
        assert_eq!(1553684, number);

        number = next_number(number);
        assert_eq!(12683156, number);

        number = next_number(number);
        assert_eq!(11100544, number);

        number = next_number(number);
        assert_eq!(12249484, number);

        number = next_number(number);
        assert_eq!(7753432, number);

        number = next_number(number);
        assert_eq!(5908254, number);
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
