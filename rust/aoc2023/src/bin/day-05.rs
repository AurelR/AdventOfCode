type NumTy = u64;
use nom::character::complete::u64 as num_parser;
use std::ops::Range;

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
        .map(|c| {
            mappings
                .iter()
                .fold(vec![c[0]..c[0] + c[1]], |ra, m| m.map_ranges(ra))
                .into_iter()
                .map(|r| r.start)
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
        .to_string()
}

#[derive(Debug)]
struct SingleMapping {
    range: Range<NumTy>,
    destination: NumTy,
}

#[derive(Debug)]
struct Mapping {
    maps: Vec<SingleMapping>,
}

impl SingleMapping {
    fn map(&self, num: NumTy) -> Option<NumTy> {
        if self.range.contains(&num) {
            Some(num - self.range.start + self.destination)
        } else {
            None
        }
    }

    fn map_range(&self, range: Range<NumTy>) -> (Vec<Range<NumTy>>, Option<Range<NumTy>>) {
        let (r1, transform, r2) = intersect(&self.range, &range);
        (
            [r1, r2].into_iter().flatten().collect(),
            transform.map(|t| {
                t.start - self.range.start + self.destination
                    ..t.end - self.range.start + self.destination
            }),
        )
    }
}

impl Mapping {
    fn map(&self, num: NumTy) -> NumTy {
        self.maps
            .iter()
            .find_map(|single_map| single_map.map(num))
            .unwrap_or(num)
    }

    fn map_ranges(&self, ranges: Vec<Range<NumTy>>) -> Vec<Range<NumTy>> {
        let mut result = Vec::new();
        for range in ranges {
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
        }
        result
    }
}

fn intersect(
    base: &Range<NumTy>,
    other: &Range<NumTy>,
) -> (
    Option<Range<NumTy>>,
    Option<Range<NumTy>>,
    Option<Range<NumTy>>,
) {
    if other.end <= base.start {
        (Some(other.clone()), None, None)
    } else if base.end <= other.start {
        (None, None, Some(other.clone()))
    } else if base.start <= other.start && other.end <= base.end {
        (None, Some(other.clone()), None)
    } else if other.start < base.start && other.end <= base.end {
        (
            Some(other.start..base.start),
            Some(base.start..other.end),
            None,
        )
    } else if base.start <= other.start && base.end < other.end {
        (None, Some(other.start..base.end), Some(base.end..other.end))
    } else {
        (
            Some(other.start..base.start),
            Some(base.clone()),
            Some(base.end..other.end),
        )
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
                        range: (source..source + length),
                        destination,
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
            range: (98..98 + 2),
            destination: 50,
        };
        assert_eq!(None, mapping.map(97));
        assert_eq!(Some(50), mapping.map(98));
        assert_eq!(Some(51), mapping.map(99));
        assert_eq!(None, mapping.map(100));
    }

    #[test]
    fn test_single_mapping_2() {
        let mapping = SingleMapping {
            range: (50..50 + 48),
            destination: 52,
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
                    range: (50..50 + 48),
                    destination: 52,
                },
                SingleMapping {
                    range: (98..98 + 2),
                    destination: 50,
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
            range: (98..98 + 10),
            destination: 50,
        };

        // total overlap
        assert_eq!((vec![], Some(50..50 + 10)), mapping.map_range(98..98 + 10));

        // inner overlap
        assert_eq!((vec![], Some(52..52 + 4)), mapping.map_range(100..100 + 4));

        // before
        assert_eq!((vec![20..20 + 23], None,), mapping.map_range(20..20 + 23));

        // after
        assert_eq!((vec![108..108 + 7], None,), mapping.map_range(108..108 + 7));

        // begin overlap
        assert_eq!(
            (vec![95..95 + 3], Some(50..50 + 4)),
            mapping.map_range(95..95 + 7)
        );

        // end overlap
        assert_eq!(
            (vec![108..108 + 4], Some(52..52 + 8)),
            mapping.map_range(100..100 + 12)
        );

        // more than total overlap
        assert_eq!(
            (vec![95..95 + 3, 108..108 + 7], Some(50..50 + 10),),
            mapping.map_range(95..95 + 20)
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
