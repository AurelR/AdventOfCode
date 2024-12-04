use std::collections::BTreeMap;
type NumTy = i32;
type Map = BTreeMap<(NumTy, NumTy), u8>;

fn main() {
    let input = std::fs::read_to_string("data/input/input04.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let mut grid = Map::new();
    let mut result = 0;
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.bytes().enumerate() {
            grid.insert((x as NumTy, y as NumTy), c);
        }
    }

    for (&pos, &c) in &grid {
        let chars = [b'M', b'A', b'S'];
        if c != b'X' {
            continue;
        }
        'dirs: for d in Direction::all() {
            let mut p = d.next_pos(pos);
            for n_c in chars {
                match grid.get(&p) {
                    Some(t_c) if *t_c == n_c => {
                        p = d.next_pos(p);
                    }
                    _ => continue 'dirs,
                }
            }
            result += 1;
        }
    }
    result.to_string()
}

fn part2(input: &str) -> String {
    let mut grid = Map::new();
    let mut result = 0;
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.bytes().enumerate() {
            grid.insert((x as NumTy, y as NumTy), c);
        }
    }

    for (&pos, &c) in &grid {
        let dirs = [
            Direction::TopRight,
            Direction::TopLeft,
            Direction::DownLeft,
            Direction::DownRight,
        ];
        if c != b'A' {
            continue;
        }
        for d in dirs {
            match grid.get(&d.next_pos(pos)) {
                Some(b'M') => {}
                _ => continue,
            }

            match grid.get(&d.rotate_90().next_pos(pos)) {
                Some(b'M') => {}
                _ => continue,
            }

            match grid.get(&d.rotate_90().rotate_90().next_pos(pos)) {
                Some(b'S') => {}
                _ => continue,
            }

            match grid.get(&d.rotate_90().rotate_90().rotate_90().next_pos(pos)) {
                Some(b'S') => {}
                _ => continue,
            }
            result += 1;
        }
    }
    result.to_string()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    TopRight,
    Top,
    TopLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
}

impl Direction {
    fn next_pos(&self, (x, y): (NumTy, NumTy)) -> (NumTy, NumTy) {
        match self {
            Direction::Right => (x + 1, y),
            Direction::TopRight => (x + 1, y - 1),
            Direction::Top => (x, y - 1),
            Direction::TopLeft => (x - 1, y - 1),
            Direction::Left => (x - 1, y),
            Direction::DownLeft => (x - 1, y + 1),
            Direction::Down => (x, y + 1),
            Direction::DownRight => (x + 1, y + 1),
        }
    }

    fn all() -> std::slice::Iter<'static, Direction> {
        use Direction::*;
        static DIRECTIONS: [Direction; 8] = [
            Right, TopRight, Top, TopLeft, Left, DownLeft, Down, DownRight,
        ];
        DIRECTIONS.iter()
    }

    fn rotate_90(&self) -> Direction {
        match self {
            Direction::Right => Direction::Top,
            Direction::TopRight => Direction::TopLeft,
            Direction::Top => Direction::Left,
            Direction::TopLeft => Direction::DownLeft,
            Direction::Left => Direction::Down,
            Direction::DownLeft => Direction::DownRight,
            Direction::Down => Direction::Right,
            Direction::DownRight => Direction::TopRight,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

        let result = part1(input);
        assert_eq!("18", result);
    }

    #[test]
    fn test_part2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

        let result = part2(input);
        assert_eq!("9", result);
    }
}
