use std::collections::BTreeSet;

type NumTy = i32;
type Pos = (NumTy, NumTy);
type Set = BTreeSet<Pos>;

fn main() {
    let input = std::fs::read_to_string("data/input/input06.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (mut pos, mut dir, max, obstructions) = parse(input);
    let rx = 0..=max.0;
    let ry = 0..=max.1;

    let mut visited = Set::new();
    loop {
        visited.insert(pos);
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if obstructions.contains(&new_pos) {
            dir = rotate_right(dir);
        } else {
            pos = new_pos;
        }
        if !rx.contains(&pos.0) || !ry.contains(&pos.1) {
            break;
        }
    }

    visited.len().to_string()
}

fn part2(input: &str) -> String {
    let (start_pos, start_dir, max, obstructions) = parse(input);
    let rx = 0..=max.0;
    let ry = 0..=max.1;

    let mut visited = Set::new();
    let mut pos = start_pos;
    let mut dir = start_dir;
    loop {
        visited.insert(pos);
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if obstructions.contains(&new_pos) {
            dir = rotate_right(dir);
        } else {
            pos = new_pos;
        }
        if !rx.contains(&pos.0) || !ry.contains(&pos.1) {
            break;
        }
    }
    visited.remove(&start_pos);
    let mut result = Vec::new();
    for o in &visited {
        let mut obs = obstructions.clone();
        obs.insert(*o);
        if find_loop(start_pos, start_dir, max, &obs) {
            result.push(*o);
        }
    }
    result.len().to_string()
}

fn parse(input: &str) -> (Pos, Pos, Pos, Set) {
    let mut start = (0, 0);
    let dir = (0, -1);
    let mut x_max = 0;
    let mut y_max = 0;
    let mut obstructions = Set::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            match c {
                b'.' => {}
                b'#' => {
                    obstructions.insert((x as NumTy, y as NumTy));
                }
                b'^' => start = (x as NumTy, y as NumTy),
                _ => panic!("Invalid Input"),
            }
            x_max = x_max.max(x as NumTy);
        }
        y_max = y_max.max(y as NumTy);
    }
    (start, dir, (x_max, y_max), obstructions)
}

fn rotate_right(dir: Pos) -> Pos {
    (-dir.1, dir.0)
}

fn find_loop(mut pos: Pos, mut dir: Pos, max: Pos, obstructions: &Set) -> bool {
    let rx = 0..=max.0;
    let ry = 0..=max.1;
    let mut visited = BTreeSet::<(Pos, Pos)>::new();
    while visited.insert((pos, dir)) {
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if obstructions.contains(&new_pos) {
            dir = rotate_right(dir);
        } else {
            pos = new_pos;
        }

        if !rx.contains(&pos.0) || !ry.contains(&pos.1) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

        let result = part1(input);
        assert_eq!("41", result);
    }

    #[test]
    fn test_part2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

        let result = part2(input);
        assert_eq!("6", result);
    }
}
