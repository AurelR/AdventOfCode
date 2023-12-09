type NumTy = i32;
use std::{cmp::Ordering, collections::BTreeMap};

use nom::character::complete::i32 as num_parser;

fn main() {
    let input = std::fs::read_to_string("data/input/input07.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let mut hands = parse_input(input).unwrap().1;
    hands.sort_by(|a, b| a.cmp1(b));
    hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid as usize)
        .sum::<usize>()
        .to_string()
}

fn part2(input: &str) -> String {
    let mut hands = parse_input(input).unwrap().1;
    hands.sort_by(|a, b| a.cmp2(b));
    hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid as usize)
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<Hand>> {
    use nom::character::complete::{newline, space1};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::{separated_pair, tuple};

    separated_list1(
        newline,
        map(
            separated_pair(
                tuple((parse_card, parse_card, parse_card, parse_card, parse_card)),
                space1,
                num_parser,
            ),
            Hand::from_tuple,
        ),
    )(input)
}

fn parse_card(input: &str) -> nom::IResult<&str, Card> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::combinator::value;
    alt((
        value(Card::A, tag("A")),
        value(Card::K, tag("K")),
        value(Card::Q, tag("Q")),
        value(Card::J, tag("J")),
        value(Card::T, tag("T")),
        value(Card::N9, tag("9")),
        value(Card::N8, tag("8")),
        value(Card::N7, tag("7")),
        value(Card::N6, tag("6")),
        value(Card::N5, tag("5")),
        value(Card::N4, tag("4")),
        value(Card::N3, tag("3")),
        value(Card::N2, tag("2")),
    ))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandValue {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Debug, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
    hand_value1: HandValue,
    hand_value2: HandValue,
    bid: NumTy,
}

impl HandValue {
    fn from_counts(counts: &[NumTy]) -> Self {
        match counts {
            [5] => HandValue::Five,
            [4, 1] => HandValue::Four,
            [3, 2] => HandValue::FullHouse,
            [3, 1, 1] => HandValue::Three,
            [2, 2, 1] => HandValue::TwoPair,
            [2, 1, 1, 1] => HandValue::OnePair,
            [1, 1, 1, 1, 1] => HandValue::HighCard,
            _ => panic!("Invalid Counts"),
        }
    }
}

impl Hand {
    fn from_tuple(((c1, c2, c3, c4, c5), bid): ((Card, Card, Card, Card, Card), NumTy)) -> Hand {
        let cards = [c1, c2, c3, c4, c5];
        let mut counts1 = BTreeMap::new();
        let mut counts2 = BTreeMap::new();
        let mut jokers = 0;
        for c in cards {
            *counts1.entry(c).or_insert(0) += 1;
            if c == Card::J {
                jokers += 1;
            } else {
                *counts2.entry(c).or_insert(0) += 1;
            }
        }
        let mut counts1 = counts1.values().copied().collect::<Vec<_>>();
        counts1.sort_by(|e1, e2| e2.cmp(e1));
        let mut counts2 = counts2.values().copied().collect::<Vec<_>>();
        counts2.sort_by(|e1, e2| e2.cmp(e1));
        if jokers != 5 {
            counts2[0] += jokers;
        } else {
            counts2.push(jokers);
        }

        Hand {
            cards,
            hand_value1: HandValue::from_counts(&counts1),
            hand_value2: HandValue::from_counts(&counts2),
            bid,
        }
    }
}

impl Hand {
    fn cmp1(&self, other: &Self) -> Ordering {
        (self.hand_value1, self.cards).cmp(&(other.hand_value1, other.cards))
    }

    fn cmp2(&self, other: &Self) -> Ordering {
        (
            self.hand_value2,
            self.cards[0].value2(),
            self.cards[1].value2(),
            self.cards[2].value2(),
            self.cards[3].value2(),
            self.cards[4].value2(),
        )
            .cmp(&(
                other.hand_value2,
                other.cards[0].value2(),
                other.cards[1].value2(),
                other.cards[2].value2(),
                other.cards[3].value2(),
                other.cards[4].value2(),
            ))
    }
}

impl Card {
    fn value2(&self) -> NumTy {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 1,
            Card::T => 10,
            Card::N9 => 9,
            Card::N8 => 8,
            Card::N7 => 7,
            Card::N6 => 6,
            Card::N5 => 5,
            Card::N4 => 4,
            Card::N3 => 3,
            Card::N2 => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        let result = part1(input);
        assert_eq!(result, "6440");
    }

    #[test]
    fn test_part2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        let result = part2(input);
        assert_eq!(result, "5905");
    }
}
