type NumTy = u64;

use nom::character::complete::u64 as int;

fn main() {
    let input = std::fs::read_to_string("data/input/input17.txt").unwrap();
    let result1 = part1(&input);
    println!("{}", result1);
    let result2 = part2(&input);
    println!("{}", result2);
}

fn part1(input: &str) -> String {
    let (reg_a, reg_b, reg_c, instructions) = parse(input).unwrap().1;
    let output = run_program(&instructions, reg_a, reg_b, reg_c);
    output
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn run_program(
    instructions: &Vec<NumTy>,
    mut reg_a: NumTy,
    mut reg_b: NumTy,
    mut reg_c: NumTy,
) -> Vec<NumTy> {
    let mut output = Vec::<NumTy>::new();
    let mut ip = 0;
    while let Some(&instr) = instructions.get(ip) {
        let &operand = instructions.get(ip + 1).unwrap();
        let result = perform_instruction(reg_a, reg_b, reg_c, ip, instr, operand, &mut output);
        reg_a = result.0;
        reg_b = result.1;
        reg_c = result.2;
        ip = result.3;
    }
    output
}

fn part2(input: &str) -> String {
    let (_reg_a, reg_b, reg_c, instructions) = parse(input).unwrap().1;

    // only works for my input, checked longer suffixes step by step
    for i in 1000..100000000 {
        for l in [0o132, 0o522] {
            let j = 0o621;
            for k in [0o633, 0o635] {
                let reg_a = i<< 27 | l << 18 | j << 9 | k;
                let output = run_program(&instructions, reg_a, reg_b, reg_c);
                // if instructions[0..13] == output[0..13] {
                //     println!("{reg_a:>16} {reg_a:>16o} {:?}", output);
                // }
                if output == instructions {
                    return reg_a.to_string();
                }
            }
        }
    }
    "".to_string()
}

fn parse(input: &str) -> nom::IResult<&str, (NumTy, NumTy, NumTy, Vec<NumTy>)> {
    use nom::bytes::complete::tag;
    use nom::multi::separated_list1;
    use nom::sequence::tuple;
    use nom::{character::complete::newline, sequence::delimited};

    tuple((
        delimited(tag("Register A: "), int, newline),
        delimited(tag("Register B: "), int, newline),
        delimited(tag("Register C: "), int, newline),
        delimited(tag("\nProgram: "), separated_list1(tag(","), int), newline),
    ))(input)
}

fn combo_value(reg_a: NumTy, reg_b: NumTy, reg_c: NumTy, operand: NumTy) -> NumTy {
    if operand <= 3 {
        operand
    } else if operand == 4 {
        reg_a
    } else if operand == 5 {
        reg_b
    } else if operand == 6 {
        reg_c
    } else {
        panic!("Invalid combo operand {operand}");
    }
}

fn perform_instruction(
    mut reg_a: NumTy,
    mut reg_b: NumTy,
    mut reg_c: NumTy,
    ip: usize,
    instr: NumTy,
    operand: NumTy,
    output: &mut Vec<NumTy>,
) -> (NumTy, NumTy, NumTy, usize) {
    match instr {
        // adv
        0 => {
            let o = combo_value(reg_a, reg_b, reg_c, operand);
            reg_a = reg_a / 2u64.pow(o as u32);
        }
        // bxl
        1 => {
            reg_b = reg_b ^ operand;
        }
        // bst
        2 => {
            reg_b = combo_value(reg_a, reg_b, reg_c, operand) % 8;
        }
        //jnz
        3 => {
            if reg_a != 0 {
                return (reg_a, reg_b, reg_c, operand as usize);
            }
        }
        // bxc
        4 => {
            reg_b = reg_b ^ reg_c;
        }
        // out
        5 => {
            output.push(combo_value(reg_a, reg_b, reg_c, operand) % 8);
        }
        // bdv
        6 => {
            let o = combo_value(reg_a, reg_b, reg_c, operand);
            reg_b = reg_a / 2u64.pow(o as u32);
        }
        // cdv
        7 => {
            let o = combo_value(reg_a, reg_b, reg_c, operand);
            reg_c = reg_a / 2u64.pow(o as u32);
        }
        _ => unimplemented!("Illegal instruction {instr} at {ip}"),
    }
    (reg_a, reg_b, reg_c, ip + 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

        let result = part1(input);
        assert_eq!("4,6,3,5,6,3,5,2,1,0", result);
    }

    #[test]
    fn test_part2() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

        let result = part2(input);
        assert_eq!("117440", result);
    }

    #[test]
    fn test_instr_1() {
        let mut output = Vec::new();

        let (reg_a, reg_b, reg_c, ip) = perform_instruction(0, 0, 9, 0, 2, 6, &mut output);
        assert_eq!(0, reg_a);
        assert_eq!(1, reg_b);
        assert_eq!(9, reg_c);
        assert_eq!(2, ip);
        assert_eq!(output, []);
    }

    #[test]
    fn test_instr_2() {
        let input = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
";

        let result = part1(input);
        assert_eq!("0,1,2", result);
    }

    #[test]
    fn test_instr_3() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

        let result = part1(input);
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0", result);
    }

    #[test]
    fn test_instr_4() {
        let mut output = Vec::new();

        let (reg_a, reg_b, reg_c, ip) = perform_instruction(0, 29, 0, 0, 1, 7, &mut output);
        assert_eq!(0, reg_a);
        assert_eq!(26, reg_b);
        assert_eq!(0, reg_c);
        assert_eq!(2, ip);
        assert_eq!(output, []);
    }

    #[test]
    fn test_instr_5() {
        let mut output = Vec::new();

        let (reg_a, reg_b, reg_c, ip) = perform_instruction(0, 2024, 43690, 0, 4, 0, &mut output);
        assert_eq!(0, reg_a);
        assert_eq!(44354, reg_b);
        assert_eq!(43690, reg_c);
        assert_eq!(2, ip);
        assert_eq!(output, []);
    }
}
