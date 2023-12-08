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
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    N9 = 9,
    N8 = 8,
    N7 = 7,
    N6 = 6,
    N5 = 5,
    N4 = 4,
    N3 = 3,
    N2 = 2,
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

impl Hand {
    fn from_tuple(((c1, c2, c3, c4, c5), bid): ((Card, Card, Card, Card, Card), NumTy)) -> Hand {
        let cards = [c1, c2, c3, c4, c5];
        let mut counts1 = BTreeMap::new();
        for c in cards {
            *counts1.entry(c).or_insert(0) += 1
        }
        let hand_value1 = match counts1.len() {
            5 => HandValue::HighCard,
            4 => HandValue::OnePair,
            3 => {
                if *counts1.values().max().unwrap() == 3 {
                    HandValue::Three
                } else {
                    HandValue::TwoPair
                }
            }
            2 => match counts1.values().next().unwrap() {
                4 | 1 => HandValue::Four,
                3 | 2 => HandValue::FullHouse,
                _ => unreachable!(),
            },
            1 => HandValue::Five,
            _ => unreachable!(),
        };

        let mut counts2 = BTreeMap::new();
        let mut jokers = 0;
        for c in cards {
            if c == Card::J {
                jokers += 1;
            } else {
                *counts2.entry(c).or_insert(0) += 1
            }
        }

        let hand_value2 = match jokers {
            5 => HandValue::Five,
            4 => HandValue::Five,
            3 => match counts2.len() {
                1 => HandValue::Five,
                2 => HandValue::Four,
                _ => unreachable!(),
            },
            2 => match counts2.len() {
                1 => HandValue::Five,
                2 => HandValue::Four,
                3 => HandValue::Three,
                _ => unreachable!(),
            },
            1 => match counts2.len() {
                1 => HandValue::Five,
                2 => {
                    if *counts2.values().max().unwrap() == 3 {
                        HandValue::Four
                    } else {
                        HandValue::FullHouse
                    }
                }
                3 => HandValue::Three,
                4 => HandValue::OnePair,
                _ => unreachable!(),
            },
            0 => hand_value1,
            _ => unreachable!(),
        };

        Hand {
            cards,
            hand_value1,
            hand_value2,
            bid,
        }
    }
}

impl Hand {
    fn cmp1(&self, other: &Self) -> Ordering {
        match self.hand_value1.cmp(&other.hand_value1) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            o => o,
        }
    }

    fn cmp2(&self, other: &Self) -> Ordering {
        match self.hand_value2.cmp(&other.hand_value2) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(c1, c2)| match c1.value2().cmp(&c2.value2()) {
                    Ordering::Equal => None,
                    o => Some(o),
                })
                .unwrap_or(Ordering::Equal),
            o => o,
        }
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
