use itertools::Itertools;
use std::{
    collections::{BTreeMap, BTreeSet},
    iter::successors,
};

type NumTy = i64;
type SequenceMap = BTreeMap<(NumTy, NumTy, NumTy, NumTy), NumTy>;

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

fn part2(input: &str) -> String {
    let numbers = parse(input);
    let data = numbers
        .into_iter()
        .enumerate()
        .map(|(i, num)| {
            let smap = successors(Some(num), |n| Some(next_number(*n)))
                .take(2001)
                .map(|n| n % 10)
                .tuple_windows()
                .map(|(n0, n1, n2, n3, n4)| ((n1 - n0, n2 - n1, n3 - n2, n4 - n3), n4))
                .fold(SequenceMap::new(), |mut map, (k, v)| {
                    map.entry(k).or_insert(v);
                    map
                });
            (i, num, smap)
        })
        .collect::<Vec<_>>();

    let sequences = data
        .iter()
        .map(|(_i, _num, smap)| smap.keys().copied().collect::<BTreeSet<_>>())
        .reduce(|last, next| last.union(&next).copied().collect())
        .unwrap();

    sequences
        .iter()
        .map(|s| {
            data.iter()
                .map(|(_i, _num, smap)| *smap.get(s).unwrap_or(&0))
                .sum::<NumTy>()
        })
        .max()
        .unwrap()
        .to_string()
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
    fn test_part2() {
        let input = "1
2
3
2024
";

        let result = part2(input);
        assert_eq!("23", result);
    }
}
