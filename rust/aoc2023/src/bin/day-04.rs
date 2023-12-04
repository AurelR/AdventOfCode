type NumTy = u32;
use nom::character::complete::u32 as NumParser;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = std::fs::read_to_string("data/input/input04.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

#[derive(Debug)]
struct Scratchcard {
    id: NumTy,
    winning_numbers: BTreeSet<NumTy>,
    my_numbers: BTreeSet<NumTy>,
}

impl Scratchcard {
    fn points(&self) -> NumTy {
        let count = self.matches();
        if count == 0 {
            0
        } else {
            (2 as NumTy).pow(count - 1)
        }
    }

    fn matches(&self) -> NumTy {
        self.winning_numbers.intersection(&self.my_numbers).count() as NumTy
    }
}

fn part1(input: &str) -> String {
    let scratchcards = parse_input(input).unwrap().1;
    scratchcards
        .iter()
        .map(Scratchcard::points)
        .sum::<NumTy>()
        .to_string()
}

fn part2(input: &str) -> String {
    let scratchcards = parse_input(input).unwrap().1;
    let mut total_cards = BTreeMap::new();
    let max_id = scratchcards.iter().last().unwrap().id;
    for card in scratchcards {
        let matches = card.matches();
        let inc = *total_cards
            .entry(card.id)
            .and_modify(|c| *c += 1)
            .or_insert(1);
        for c in card.id + 1..=(card.id + matches).min(max_id) {
            *total_cards.entry(c).or_insert(0) += inc;
        }
    }
    total_cards.into_values().sum::<NumTy>().to_string()
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<Scratchcard>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::newline;
    use nom::character::complete::space1;
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair};

    separated_list1(
        newline,
        map(
            separated_pair(
                preceded(tag("Card"), preceded(space1, NumParser)),
                tag(":"),
                separated_pair(
                    preceded(space1, parse_number_list),
                    tag(" |"),
                    preceded(space1, parse_number_list),
                ),
            ),
            |(id, (winning_numbers, my_numbers))| Scratchcard {
                id,
                winning_numbers,
                my_numbers,
            },
        ),
    )(input)
}

fn parse_number_list(input: &str) -> nom::IResult<&str, BTreeSet<NumTy>> {
    use nom::character::complete::space1;
    use nom::combinator::map;
    use nom::multi::separated_list1;
    map(separated_list1(space1, NumParser), |numbers| {
        BTreeSet::from_iter(numbers)
    })(input)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let result = part1(input);
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let result = part2(input);
        assert_eq!(result, "30");
    }
}
