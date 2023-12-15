type NumTy = i32;

fn main() {
    let input = std::fs::read_to_string("data/input/input15.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let data = parse_input(input);
    data.into_iter()
        .map(|s| {
            s.as_bytes()
                .into_iter()
                .fold(0, |c, n| ((c + (*n as NumTy)) * 17) % 256)
        })
        .sum::<NumTy>()
        .to_string()
}

fn part2(input: &str) -> String {
    "".to_string()
}

#[allow(unused_imports)]
fn parse_input(input: &str) -> Vec<&str> {
    input.lines().flat_map(|line| line.split(",")).collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";
        let result = part1(input);
        assert_eq!(result, "1320");
    }

    #[test]
    fn test_part2() {
        let input = "\
";
        let result = part2(input);
        assert_eq!(result, "todo");
    }
}
