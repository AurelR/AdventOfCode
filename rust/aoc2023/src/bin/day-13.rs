type NumTy = usize;

fn main() {
    let input = std::fs::read_to_string("data/input/input13.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let data = parse_input(input).unwrap().1;
    data.iter().map(Grid::score1).sum::<NumTy>().to_string()
}

fn part2(input: &str) -> String {
    let data = parse_input(input).unwrap().1;
    data.iter().map(Grid::score2).sum::<NumTy>().to_string()
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<Grid>> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, newline};
    use nom::combinator::{map, value};
    use nom::multi::{many1, separated_list1};

    separated_list1(
        tag("\n\n"),
        map(
            separated_list1(
                newline,
                many1(alt((
                    value(Location::Ash, char('.')),
                    value(Location::Rock, char('#')),
                ))),
            ),
            |contents| Grid {
                x_size: contents[0].len(),
                y_size: contents.len(),
                contents,
            },
        ),
    )(input)
}

#[derive(Debug, Clone)]
struct Grid {
    contents: Vec<Vec<Location>>,
    x_size: usize,
    y_size: usize,
}

impl Grid {
    fn score1(&self) -> NumTy {
        self.horizontal_reflection().map_or_else(
            || self.transpose().horizontal_reflection().unwrap_or(0),
            |s| s * 100,
        )
    }

    fn score2(&self) -> NumTy {
        self.horizontal_miss_1().map_or_else(
            || self.transpose().horizontal_miss_1().unwrap_or(0),
            |s| s * 100,
        )
    }

    fn horizontal_reflection(&self) -> Option<NumTy> {
        for i in 1..self.y_size {
            let len = i.min(self.y_size / 2).min(self.y_size - i);
            let a = &self.contents[i - len..i];
            let b = &self.contents[i..i + len];
            if a.iter().rev().eq(b.iter()) {
                return Some(i);
            }
        }
        None
    }

    fn horizontal_miss_1(&self) -> Option<NumTy> {
        for i in 1..self.y_size {
            let len = i.min(self.y_size / 2).min(self.y_size - i);
            let a = &self.contents[i - len..i];
            let b = &self.contents[i..i + len];
            if a.iter()
                .rev()
                .zip(b)
                .map(|(u, v)| u.iter().zip(v.iter()).filter(|(z, w)| z != w).count())
                .sum::<usize>()
                == 1
            {
                return Some(i);
            }
        }
        None
    }

    fn transpose(&self) -> Grid {
        Grid {
            x_size: self.y_size,
            y_size: self.x_size,
            contents: (0..self.x_size)
                .map(|i| {
                    self.contents
                        .iter()
                        .map(|inner| inner[i].clone())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Location {
    Ash,
    Rock,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        let result = part1(input);
        assert_eq!(result, "405");
    }

    #[test]
    fn test_part2() {
        let input = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#     
";
        let result = part2(input);
        assert_eq!(result, "400");
    }
}
