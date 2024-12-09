use std::collections::VecDeque;

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
    let mut result =  Vec::new();
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
            Kind::Used(id) => {
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
            },
        }
    }
    let mut checksum = 0;
    for slot in result {
        let Kind::Used(id) = slot.kind else {
            panic!("should not happen");
        };
        for b in slot.begin..slot.end {
            checksum += id * b;
        }
    }
    checksum.to_string()
}

fn part2(input: &str) -> String {
    let data = parse(input);
    let mut result = 0;
    result.to_string()
}

fn parse(input: &str) -> VecDeque<Slot>{
    let line = input.lines().next().unwrap();
    let mut memory = VecDeque::new();
    let mut f = true;
    let mut id = 0;
    let mut offset = 0;
    for c in line.as_bytes() {
        let len = (c - 48) as NumTy;
        if f {
            memory.push_back(Slot {kind: Kind::Used(id), begin: offset, end: offset+len});
            id += 1;
            f = !f;
        } else {
            memory.push_back(Slot {kind: Kind::Empty, begin: offset, end: offset+len});
            f = !f;
        }
        offset += len;
    }
    return memory;
}

#[derive(Debug,Clone, Copy)]
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

#[derive(Debug,Clone, Copy)]
enum Kind {
    Empty,
    Used(NumTy),
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
        let input = "
";

        let result = part2(input);
        assert_eq!("0000", result);
    }

}
