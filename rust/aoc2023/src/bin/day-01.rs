type NumTy = i32;

fn main() {
    let input = std::fs::read_to_string("data/input/input01.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| line.as_bytes().iter().filter_map(convert_char1))
        .map(summarize_line)
        .sum::<NumTy>()
        .to_string()
}

fn convert_char1(c: &u8) -> Option<NumTy> {
    match c {
        b'1'..=b'9' => Some((c - b'0') as NumTy),
        _ => None,
    }
}

fn summarize_line(mut it: impl Iterator<Item = NumTy>) -> NumTy {
    match (it.next(), it.last()) {
        (Some(n10), Some(n1)) => n10 * 10 + n1,
        (Some(n), None) => n * 10 + n,
        _ => 0,
    }
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            (0..line.len()).filter_map(|index| convert_char2(&line[index..]))
        })
        .map(summarize_line)
        .sum::<NumTy>()
        .to_string()
}

fn convert_char2(line_part: &[u8]) -> Option<NumTy> {
    if line_part[0] == b'1' || line_part.starts_with(b"one") {
        Some(1)
    } else if line_part[0] == b'2' || line_part.starts_with(b"two") {
        Some(2)
    } else if line_part[0] == b'3' || line_part.starts_with(b"three") {
        Some(3)
    } else if line_part[0] == b'4' || line_part.starts_with(b"four") {
        Some(4)
    } else if line_part[0] == b'5' || line_part.starts_with(b"five") {
        Some(5)
    } else if line_part[0] == b'6' || line_part.starts_with(b"six") {
        Some(6)
    } else if line_part[0] == b'7' || line_part.starts_with(b"seven") {
        Some(7)
    } else if line_part[0] == b'8' || line_part.starts_with(b"eight") {
        Some(8)
    } else if line_part[0] == b'9' || line_part.starts_with(b"nine") {
        Some(9)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        ";

        let result = part1(input);
        assert_eq!("142", result);
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        ";

        let result = part2(input);
        assert_eq!("281", result);
    }
}
