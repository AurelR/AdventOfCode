type NumTy = i32;

fn main() {
    let input = std::fs::read_to_string("data/input/input25.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (locks, keys) = parse(input);
    locks
        .iter()
        .map(|lock| {
            keys.iter()
                .filter(|&key| lock.iter().zip(key).all(|(l, k)| l + k <= 5))
                .count()
        })
        .sum::<usize>()
        .to_string()
}

fn part2(_input: &str) -> String {
    "".to_string()
}

fn parse(input: &str) -> (Vec<[NumTy; 5]>, Vec<[NumTy; 5]>) {
    let locks = input
        .split("\n\n")
        .filter(|&block| block.starts_with("#####"))
        .map(|lock_str| {
            lock_str.lines().skip(1).fold([0; 5], |mut lock, line| {
                line.chars().zip(&mut lock).for_each(|(c, l)| {
                    if c == '#' {
                        *l += 1
                    }
                });
                lock
            })
        })
        .collect();

    let keys = input
        .split("\n\n")
        .filter(|&block| block.starts_with("....."))
        .map(|key_str| {
            key_str.lines().take(6).fold([0; 5], |mut key, line| {
                line.chars().zip(&mut key).for_each(|(c, k)| {
                    if c == '#' {
                        *k += 1
                    }
                });
                key
            })
        })
        .collect();

    (locks, keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

        let (locks, keys) = parse(input);
        assert_eq!(locks, [[0, 5, 3, 4, 3], [1, 2, 0, 5, 3]]);
        assert_eq!(keys, [[5, 0, 2, 1, 3], [4, 3, 4, 0, 2], [3, 0, 2, 0, 1]]);
    }

    #[test]
    fn test_part1() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

        let result = part1(input);
        assert_eq!("3", result);
    }
}
