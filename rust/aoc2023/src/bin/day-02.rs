fn main() {
    let input = std::fs::read_to_string("data/input/input02.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let games = parse_input(input).unwrap().1;
    games
        .into_iter()
        .filter(Game::is_possible)
        .map(|game| game.id)
        .sum::<NumTy>()
        .to_string()
}

fn part2(input: &str) -> String {
    let games = parse_input(input).unwrap().1;
    games
        .iter()
        .map(Game::minimal_power)
        .sum::<NumTy>()
        .to_string()
}

type NumTy = u32;
use nom::character::complete::u32 as NumParser;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    id: NumTy,
    drawings: Vec<Drawing>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Drawing {
    red: NumTy,
    green: NumTy,
    blue: NumTy,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.drawings.iter().all(Drawing::is_possible)
    }

    fn minimal_power(&self) -> NumTy {
        let (red, green, blue) = self
            .drawings
            .iter()
            .fold((0, 0, 0), |(red, green, blue), d| {
                (red.max(d.red), green.max(d.green), blue.max(d.blue))
            });
        red * green * blue
    }
}

impl Drawing {
    fn is_possible(&self) -> bool {
        if self.red <= 12 && self.green <= 13 && self.blue <= 14 {
            true
        } else {
            false
        }
    }
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<Game>> {
    use nom::character::complete::newline;
    use nom::multi::separated_list1;

    separated_list1(newline, parse_game)(input)
}

fn parse_game(input: &str) -> nom::IResult<&str, Game> {
    use nom::bytes::complete::tag;
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::preceded;
    use nom::sequence::separated_pair;
    map(
        separated_pair(
            preceded(tag("Game "), NumParser),
            tag(":"),
            separated_list1(tag(";"), parse_drawing),
        ),
        |(id, drawings)| Game { id, drawings },
    )(input)
}

fn parse_drawing(input: &str) -> nom::IResult<&str, Drawing> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::combinator::map;
    use nom::combinator::value;
    use nom::multi::separated_list1;
    use nom::sequence::preceded;
    use nom::sequence::separated_pair;
    #[derive(Clone, Copy, Debug)]
    enum Color {
        Red,
        Green,
        Blue,
    }
    map(
        separated_list1(
            tag(","),
            separated_pair(
                preceded(tag(" "), NumParser),
                tag(" "),
                alt((
                    value(Color::Red, tag("red")),
                    value(Color::Green, tag("green")),
                    value(Color::Blue, tag("blue")),
                )),
            ),
        ),
        |colors| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for color in colors {
                match color {
                    (c, Color::Red) => red = c,
                    (c, Color::Green) => green = c,
                    (c, Color::Blue) => blue = c,
                }
            }
            Drawing { red, green, blue }
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let result = part1(input);
        assert_eq!(result, "8");
    }

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let result = part2(input);
        assert_eq!(result, "2286");
    }
}
