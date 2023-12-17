type NumTy = i32;
type NumPair = (NumTy, NumTy);

use pathfinding::directed::astar::astar;

fn main() {
    let input = std::fs::read_to_string("data/input/input17.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let grid = parse_input(input);

    let start = Node {
        x: 0,
        y: 0,
        last_dir: Direction::Right,
        count_dir: 0,
    };
    let target = Node {
        x: grid.x_size - 1,
        y: grid.y_size - 1,
        last_dir: Direction::Right,
        count_dir: 0,
    };
    let heatloss = astar(
        &start,
        |node| {
            node.last_dir
                .new_directions()
                .into_iter()
                .map(|new_dir| {
                    let (x, y) = new_dir.apply(node.x, node.y);
                    let count_dir = if node.last_dir == new_dir {
                        node.count_dir + 1
                    } else {
                        1
                    };
                    Node {
                        x,
                        y,
                        count_dir,
                        last_dir: new_dir,
                    }
                })
                .filter(|n| {
                    (0..grid.x_size).contains(&n.x)
                        && (0..grid.y_size).contains(&n.y)
                        && n.count_dir <= 3
                })
                .map(|n| (n, grid.contents[n.y as usize][n.x as usize]))
                .collect::<Vec<_>>()
        },
        |node| (node.x.abs_diff(target.x) + node.y.abs_diff(target.y)) as NumTy,
        |node| node.x == target.x && node.y == target.y,
    )
    .unwrap()
    .1;
    heatloss.to_string()
}

fn part2(input: &str) -> String {
    let grid = parse_input(input);

    let start = Node {
        x: 0,
        y: 0,
        last_dir: Direction::Right,
        count_dir: 0,
    };
    let target = Node {
        x: grid.x_size - 1,
        y: grid.y_size - 1,
        last_dir: Direction::Right,
        count_dir: 0,
    };
    let heatloss = astar(
        &start,
        |node| {
            let neighbors = if node == &start {
                vec![
                    (
                        Node {
                            x: 1,
                            y: 0,
                            count_dir: 1,
                            last_dir: Direction::Right,
                        },
                        grid.contents[0][1],
                    ),
                    (
                        Node {
                            x: 0,
                            y: 1,
                            count_dir: 1,
                            last_dir: Direction::Down,
                        },
                        grid.contents[1][0],
                    ),
                ]
            } else {
                node.last_dir
                    .new_directions()
                    .into_iter()
                    .filter_map(|new_dir| {
                        let (x, y) = new_dir.apply(node.x, node.y);
                        let (count_dir, keep_dir) = if node.last_dir == new_dir {
                            (node.count_dir + 1, true)
                        } else {
                            (1, false)
                        };
                        let n = Node {
                            x,
                            y,
                            count_dir,
                            last_dir: new_dir,
                        };
                        if (0..grid.x_size).contains(&n.x)
                            && (0..grid.y_size).contains(&n.y)
                            && n.count_dir <= 10
                            && (node.count_dir >= 4 || keep_dir)
                        {
                            Some(n)
                        } else {
                            None
                        }
                    })
                    .map(|n| (n, grid.contents[n.y as usize][n.x as usize]))
                    .collect::<Vec<_>>()
            };
            neighbors
        },
        |node| (node.x.abs_diff(target.x) + node.y.abs_diff(target.y)) as NumTy,
        |node| node.x == target.x && node.y == target.y && node.count_dir >= 4,
    )
    .unwrap()
    .1;
    heatloss.to_string()
}

fn parse_input(input: &str) -> Grid {
    let contents = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as NumTy)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Grid {
        y_size: contents.len() as NumTy,
        x_size: contents[0].len() as NumTy,
        contents,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    contents: Vec<Vec<NumTy>>,
    x_size: NumTy,
    y_size: NumTy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    x: NumTy,
    y: NumTy,
    last_dir: Direction,
    count_dir: NumTy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn new_directions(&self) -> [Direction; 3] {
        match self {
            Direction::Right => [Direction::Right, Direction::Up, Direction::Down],
            Direction::Up => [Direction::Up, Direction::Right, Direction::Left],
            Direction::Left => [Direction::Left, Direction::Up, Direction::Down],
            Direction::Down => [Direction::Down, Direction::Right, Direction::Left],
        }
    }

    fn apply(&self, x: NumTy, y: NumTy) -> NumPair {
        match self {
            Direction::Right => (x + 1, y),
            Direction::Up => (x, y - 1),
            Direction::Left => (x - 1, y),
            Direction::Down => (x, y + 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        let result = part1(input);
        assert_eq!(result, "102");
    }

    #[test]
     fn test_part2() {
        let input = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        let result = part2(input);
        assert_eq!(result, "94");
    }
}
