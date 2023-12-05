type NumTy = u64;
use nom::character::complete::u64 as num_parser;

fn main() {
    let input = std::fs::read_to_string("data/input/input05.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (seeds, mappings) = parse_input(input).unwrap().1;
    seeds
        .into_iter()
        .map(|seed| mappings.iter().fold(seed, |s, m| m.map(s)))
        .min()
        .unwrap()
        .to_string()
}

fn part2(input: &str) -> String {
    let (seeds, mappings) = parse_input(input).unwrap().1;
    seeds
        .chunks_exact(2)
        .map(|c| Ranges {
            ranges: vec![Range {
                begin: c[0],
                length: c[1],
            }],
        })
        .take(1)
        .map(|ranges| mappings.iter().fold(ranges, |ra, m| m.map_ranges(ra)))
        .map(|r| r.min())
        .min()
        .unwrap()
        .to_string()
}

#[derive(Debug)]
struct SingleMapping {
    source: NumTy,
    destination: NumTy,
    length: NumTy,
}

#[derive(Debug)]
struct Mapping {
    maps: Vec<SingleMapping>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    begin: NumTy,
    length: NumTy,
}

#[derive(Debug, Clone)]
struct Ranges {
    ranges: Vec<Range>,
}

impl SingleMapping {
    fn map(&self, num: NumTy) -> Option<NumTy> {
        if (self.source..self.source + self.length).contains(&num) {
            Some(num - self.source + self.destination)
        } else {
            None
        }
    }

    fn map_range(&self, range: Range) -> (Vec<Range>, Option<Range>) {
        if range.begin < self.source {
            if range.begin + range.length <= self.source {
                (vec![range], None)
            } else {
                if range.begin + range.length >= self.source + self.length {
                    (
                        vec![
                            Range {
                                begin: range.begin,
                                length: self.source - range.begin,
                            },
                            Range {
                                begin: self.source + self.length,
                                length: range.begin + range.length - (self.source + self.length),
                            },
                        ],
                        Some(Range {
                            begin: self.destination,
                            length: self.length,
                        }),
                    )
                } else {
                    (
                        vec![Range {
                            begin: range.begin,
                            length: self.source - range.begin,
                        }],
                        Some(Range {
                            begin: self.destination,
                            length: range.begin + self.length - self.source,
                        }),
                    )
                }
            }
        } else if self.source <= range.begin && range.begin < self.source + self.length {
            if range.begin + range.length <= self.source + self.length {
                (
                    vec![],
                    Some(Range {
                        begin: range.begin - self.source + self.destination,
                        length: range.length,
                    }),
                )
            } else {
                (
                    vec![Range {
                        begin: self.source + self.length,
                        length: range.begin + range.length - (self.source + self.length),
                    }],
                    Some(Range {
                        begin: range.begin - self.source + self.destination,
                        length: self.source + self.length - range.begin,
                    }),
                )
            }
        } else {
            (vec![range], None)
        }
    }
}

impl Mapping {
    fn map(&self, num: NumTy) -> NumTy {
        self.maps
            .iter()
            .find_map(|single_map| single_map.map(num))
            .unwrap_or(num)
    }

    fn map_ranges(&self, ranges: Ranges) -> Ranges {
        let mut result = Vec::new();
        'outer: for range in ranges.ranges {
            let mut sub_ranges = vec![range];
            for m in &self.maps {
                let mut new_sub_ranges = Vec::new();
                for r in sub_ranges {
                    let (mut unmapped_ranges, mapped_ranges) = m.map_range(r);
                    if let Some(mappped_range) = mapped_ranges {
                        result.push(mappped_range);
                    }
                    new_sub_ranges.append(&mut unmapped_ranges);
                }
                sub_ranges = new_sub_ranges;
            }
            result.append(&mut sub_ranges);

            //     let (unmapped_ranges, mapped_ranges) = m.map_range(r);
            //     if let Some(mappped_range) =  mapped_ranges {
            //         result.push(mappped_range)
            //     }
            //     if unmapped_ranges.is_empty() {
            //         continue 'outer;
            //     }

            //      {
            //         (None, Some(mapped_range)) => {
            //             result.push(mapped_range);

            //         }
            //         (Some(unmapped_range), None) => r = unmapped_range,
            //         (Some(unmapped_range), Some(mapped_range)) => {
            //             result.push(mapped_range);
            //             r = unmapped_range;
            //         }
            //         (None, None) => panic!("Should not happend"),
            //     }
            // }
            // result.push(r);
        }
        Ranges { ranges: result }
    }
}

impl Ranges {
    fn min(&self) -> NumTy {
        self.ranges.iter().map(|r| r.begin).min().unwrap()
    }
}

fn parse_input(input: &str) -> nom::IResult<&str, (Vec<NumTy>, Vec<Mapping>)> {
    use nom::bytes::complete::tag;
    use nom::character::complete::space1;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair};
    separated_pair(
        preceded(tag("seeds: "), separated_list1(space1, num_parser)),
        tag("\n\n"),
        separated_list1(tag("\n\n"), parse_mapping),
    )(input)
}

fn parse_mapping(input: &str) -> nom::IResult<&str, Mapping> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::newline;
    use nom::character::complete::space1;
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, tuple};

    preceded(
        alt((
            tag("seed-to-soil map:\n"),
            tag("soil-to-fertilizer map:\n"),
            tag("fertilizer-to-water map:\n"),
            tag("water-to-light map:\n"),
            tag("light-to-temperature map:\n"),
            tag("temperature-to-humidity map:\n"),
            tag("humidity-to-location map:\n"),
        )),
        map(
            separated_list1(
                newline,
                map(
                    tuple((num_parser, space1, num_parser, space1, num_parser)),
                    |(destination, _, source, _, length)| SingleMapping {
                        source,
                        destination,
                        length,
                    },
                ),
            ),
            |maps| Mapping { maps },
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_single_mapping_1() {
        let mapping = SingleMapping {
            source: 98,
            destination: 50,
            length: 2,
        };
        assert_eq!(None, mapping.map(97));
        assert_eq!(Some(50), mapping.map(98));
        assert_eq!(Some(51), mapping.map(99));
        assert_eq!(None, mapping.map(100));
    }

    #[test]
    fn test_single_mapping_2() {
        let mapping = SingleMapping {
            source: 50,
            destination: 52,
            length: 48,
        };
        assert_eq!(None, mapping.map(49));
        assert_eq!(Some(52), mapping.map(50));
        assert_eq!(Some(99), mapping.map(97));
        assert_eq!(None, mapping.map(98));
    }

    #[test]
    fn test_mapping() {
        let mapping = Mapping {
            maps: vec![
                SingleMapping {
                    source: 50,
                    destination: 52,
                    length: 48,
                },
                SingleMapping {
                    source: 98,
                    destination: 50,
                    length: 2,
                },
            ],
        };
        assert_eq!(0, mapping.map(0));
        assert_eq!(49, mapping.map(49));
        assert_eq!(52, mapping.map(50));
        assert_eq!(53, mapping.map(51));
        assert_eq!(54, mapping.map(52));
        assert_eq!(55, mapping.map(53));
        assert_eq!(99, mapping.map(97));
        assert_eq!(50, mapping.map(98));
        assert_eq!(51, mapping.map(99));
        assert_eq!(100, mapping.map(100));
    }

    #[test]
    fn test_part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        let result = part1(input);
        assert_eq!(result, "35");
    }

    #[test]
    fn test_single_mapping_map_range() {
        let mapping = SingleMapping {
            source: 98,
            destination: 50,
            length: 10,
        };

        // total overlap
        assert_eq!(
            (
                vec![],
                Some(Range {
                    begin: 50,
                    length: 10
                })
            ),
            mapping.map_range(Range {
                begin: 98,
                length: 10
            })
        );

        // inner overlap
        assert_eq!(
            (
                vec![],
                Some(Range {
                    begin: 52,
                    length: 4
                })
            ),
            mapping.map_range(Range {
                begin: 100,
                length: 4
            })
        );

        // before
        assert_eq!(
            (
                vec![Range {
                    begin: 20,
                    length: 23
                }],
                None,
            ),
            mapping.map_range(Range {
                begin: 20,
                length: 23
            })
        );

        // after
        assert_eq!(
            (
                vec![Range {
                    begin: 108,
                    length: 7
                }],
                None,
            ),
            mapping.map_range(Range {
                begin: 108,
                length: 7
            })
        );

        // more than total overlap
        assert_eq!(
            (
                vec![
                    Range {
                        begin: 95,
                        length: 3
                    },
                    Range {
                        begin: 108,
                        length: 7
                    }
                ],
                Some(Range {
                    begin: 50,
                    length: 10
                }),
            ),
            mapping.map_range(Range {
                begin: 95,
                length: 20
            })
        );
    }

    #[test]
    fn test_part2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        let result = part2(input);
        assert_eq!(result, "46");
    }
}
