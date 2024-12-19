use std::collections::{BTreeMap, VecDeque};

type NumTy = i64;

fn main() {
    let input = std::fs::read_to_string("data/input/input09.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let mut mem = parse(input);
    let mut result = Vec::new();
    let mut slot;
    let mut offset = 0;
    loop {
        slot = mem.pop_front().unwrap();

        if let Kind::Used(_) = slot.kind {
            if slot.begin == offset {
                result.push(slot);
                offset += slot.len();
            } else {
                assert!(mem.is_empty());
                let len = slot.len();
                slot.begin = offset;
                slot.end = offset + len;
                result.push(slot);
                break;
            }
            continue;
        }

        let Some(mut slot2) = mem.pop_back() else {
            break;
        };
        if let Kind::Empty = slot2.kind {
            slot2 = mem.pop_back().unwrap();
        }

        match slot2.kind {
            Kind::Empty => todo!(),
            Kind::Used(_id) => {
                if slot.len() > slot2.len() {
                    let end = slot.begin + slot2.len();
                    slot2.begin = slot.begin;
                    slot2.end = end;
                    result.push(slot2);
                    offset = end;
                    slot.begin = end;
                    mem.push_front(slot);
                } else if slot.len() < slot2.len() {
                    let len = slot.len();
                    slot.kind = slot2.kind;
                    result.push(slot);
                    slot2.begin += len;
                    offset = slot.end;
                    mem.push_back(slot2);
                } else {
                    slot.kind = slot2.kind;
                    offset = slot.end;
                    result.push(slot);
                }
            }
        }
    }
    calc_checksum(&result).to_string()
}

fn calc_checksum<'a>(result: impl IntoIterator<Item = &'a Slot>) -> NumTy {
    let mut checksum = 0;
    for slot in result {
        let Kind::Used(id) = slot.kind else {
            continue;
        };
        for b in slot.begin..slot.end {
            checksum += id * b;
        }
    }
    checksum
}

fn part2(input: &str) -> String {
    let data = parse(input);
    let mut todo = data
        .iter()
        .filter(|s| !s.kind.is_empty())
        .copied()
        .collect::<Vec<_>>();
    let mut empty = data
        .iter()
        .filter(|s| s.kind.is_empty())
        .map(|s| (s.begin, *s))
        .collect::<BTreeMap<NumTy, Slot>>();
    let mut result = Vec::<Slot>::new();
    while let Some(s) = todo.pop() {
        if let Some((&key, &target)) = empty
            .iter()
            .find(|p| *p.0 < s.begin && p.1.len() >= s.len())
        {
            let mut new_s = s.clone();
            new_s.begin = target.begin;
            new_s.end = new_s.begin + s.len();
            if s.len() < target.len() {
                let mut new_e = target.clone();
                new_e.begin += s.len();
                empty.insert(new_e.begin, new_e);
            }
            result.push(new_s);
            empty.remove(&key);
        } else {
            result.push(s);
        }
    }
    calc_checksum(&result).to_string()
}

fn parse(input: &str) -> VecDeque<Slot> {
    let line = input.lines().next().unwrap();
    let mut memory = VecDeque::new();
    let mut f = true;
    let mut id = 0;
    let mut offset = 0;
    for c in line.as_bytes() {
        let len = (c - 48) as NumTy;
        if f {
            memory.push_back(Slot {
                kind: Kind::Used(id),
                begin: offset,
                end: offset + len,
            });
            id += 1;
            f = !f;
        } else {
            memory.push_back(Slot {
                kind: Kind::Empty,
                begin: offset,
                end: offset + len,
            });
            f = !f;
        }
        offset += len;
    }
    return memory;
}

#[derive(Debug, Clone, Copy)]
struct Slot {
    kind: Kind,
    begin: NumTy,
    end: NumTy,
}

impl Slot {
    fn len(&self) -> NumTy {
        self.end - self.begin
    }
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Empty,
    Used(NumTy),
}

impl Kind {
    fn is_empty(&self) -> bool {
        match self {
            Kind::Empty => true,
            Kind::Used(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = "2333133121414131402
";

        let result = part1(input);
        assert_eq!("1928", result);
    }

    #[test]
    fn test_part2() {
        let input = "2333133121414131402
";

        let result = part2(input);
        assert_eq!("2858", result);
    }
}
