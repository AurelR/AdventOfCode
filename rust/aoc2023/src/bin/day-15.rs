type NumTy = usize;

fn main() {
    let input = std::fs::read_to_string("data/input/input15.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let data = parse_input(input);
    data.into_iter().map(hash).sum::<NumTy>().to_string()
}

fn part2(input: &str) -> String {
    let data = parse_input(input);
    let mut table: Vec<Vec<(&str, NumTy)>> = vec![vec![]; 256];
    for &op in &data {
        if let Some((label, _)) = op.split_once("-") {
            let h = hash(label);
            table[h].retain(|(l, _)| *l != label);
        } else if let Some((label, lens)) = op.split_once("=") {
            let h = hash(label);
            let contents = &mut table[h];
            match contents.iter_mut().find(|(l, _)| *l == label) {
                Some((_, old_lens)) => {
                    *old_lens = lens.parse().unwrap();
                }
                None => contents.push((label, lens.parse().unwrap())),
            }
        } else {
            unreachable!("No marker in string: {op}");
        }
    }

    table
        .into_iter()
        .enumerate()
        .flat_map(|(box_num, box_contents)| {
            box_contents
                .into_iter()
                .enumerate()
                .map(move |(slot_num, (_, lens))| (box_num + 1) * (slot_num + 1) * lens)
        })
        .sum::<NumTy>()
        .to_string()
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().flat_map(|line| line.split(",")).collect()
}

fn hash(input: &str) -> NumTy {
    input
        .as_bytes()
        .into_iter()
        .fold(0, |c, n| ((c + (*n as NumTy)) * 17) % 256)
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
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";
        let result = part2(input);
        assert_eq!(result, "145");
    }
}
