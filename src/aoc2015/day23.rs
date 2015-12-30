

enum Instruction {
    Half(usize),
    Triple(usize),
    Increment(usize),
    Jump(isize),
    JumpEven{reg: usize, offset: isize},
    JumpOne{reg: usize, offset: isize},
}

pub fn solve(input: &String) -> Vec<Result<usize, String>> {
    let instructions = parse_instructions(input);
    let p1 = run(&instructions, &[0; 2]);
    let p2 = run(&instructions, &[1, 0]);
    vec![Ok(p1[1]), Ok(p2[1])]
}

fn run(instructions: &Vec<Instruction>, initial: &[usize; 2]) -> [usize; 2] {
    use self::Instruction::*;
    let mut i = 0;
    let mut reg = initial.clone();
    while i >= 0 && i < instructions.len() as isize {
        let mut step = 1;
        match instructions[i as usize] {
            Half(r) => reg[r] /= 2,
            Triple(r) => reg[r] *= 3,
            Increment(r) => reg[r] += 1,
            Jump(offset) => step = offset,
            JumpEven{reg: r, offset} => if reg[r] % 2 == 0 { step = offset},
            JumpOne{reg: r, offset} => if reg[r] == 1 {step = offset},
        }
        i += step;
    }
    reg
}

fn parse_instructions(input: &String) -> Vec<Instruction> {
    use self::Instruction::*;
    let mut v = vec![];
    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<_>>();
        let i = match split[0] {
            "hlf" => Half(get_reg(split[1])),
            "tpl" => Triple(get_reg(split[1])),
            "inc" => Increment(get_reg(split[1])),
            "jmp" => Jump(get_offset(split[1])),
            "jie" => JumpEven{reg: get_reg(split[1]), offset: get_offset(split[2])},
            "jio" => JumpOne{reg: get_reg(split[1]), offset: get_offset(split[2])},
            _ => panic!("unrecognised instruction")
        };
        v.push(i);
    }
    v
}

fn get_reg(reg: &str) -> usize {
    if reg.starts_with("a") { 0 
    } else if reg.starts_with("b") { 1
    } else { panic!("unrecognised reg")
    }
}

fn get_offset(offset: &str) -> isize {
    offset.replace("+", "").parse::<isize>().expect("Offset not a number")
}
