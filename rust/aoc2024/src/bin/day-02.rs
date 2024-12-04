use itertools::Itertools;
type NumTy = i32;

fn main() {
    let input = std::fs::read_to_string("data/input/input02.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let mut safe = 0;
    for l in input.lines() {
        let nums = l
            .split_ascii_whitespace()
            .map(|n| n.parse::<NumTy>().unwrap())
            .collect::<Vec<_>>();
        if is_safe(&nums) {
            safe += 1;
        }
    }
    safe.to_string()
}

fn part2(input: &str) -> String {
    let mut safe = 0;
    'outer: for l in input.lines() {
        let nums = l
            .split_ascii_whitespace()
            .map(|n| n.parse::<NumTy>().unwrap())
            .collect::<Vec<_>>();
        if is_safe(&nums) {
            safe += 1;
            continue;
        }
        for d in 0..nums.len() {
            let mut n2 = nums.clone();
            n2.remove(d);
            if is_safe(&n2) {
                safe += 1;
                continue 'outer;
            }
        }
    }
    safe.to_string()
}

fn is_safe(nums: &[NumTy]) -> bool {
    let mut min_diff = 1000000;
    let mut max_diff = 0;
    let mut inc = false;
    let mut dec = false;
    for (a, b) in nums.iter().tuple_windows() {
        let diff = a.abs_diff(*b);
        min_diff = min_diff.min(diff);
        max_diff = max_diff.max(diff);
        match a.cmp(b) {
            std::cmp::Ordering::Less => dec = true,
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => inc = true,
        }
    }
    if min_diff >= 1 && max_diff <= 3 && (inc ^ dec) {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

        let result = part1(input);
        assert_eq!("2", result);
    }

    #[test]
    fn test_part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

        let result = part2(input);
        assert_eq!("4", result);
    }
}
