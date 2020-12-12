#[derive(Clone, Copy)]
pub enum InstrKind {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

#[derive(Clone, Copy)]
pub struct Instruction {
    kind: InstrKind,
    visited: bool,
}

#[aoc_generator(day8)]
pub fn gen(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|s| {
            let (kind, val) = s.split_at(s.find(' ').unwrap());
            let val = val.trim().parse().unwrap();
            let kind = match kind.trim() {
                "nop" => InstrKind::Nop(val),
                "acc" => InstrKind::Acc(val),
                "jmp" => InstrKind::Jmp(val),
                s @ _ => panic!("Unexpected instruction `{}`", s),
            };

            Instruction {
                kind,
                visited: false,
            }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part1_impl1(input: &Vec<Instruction>) -> i64 {
    let mut input = input.clone();
    let mut acc = 0;
    let mut pc = 0i64;

    while !input[pc as usize].visited {
        let instr = input.get_mut(pc as usize).unwrap();
        match instr.kind {
            InstrKind::Nop(_) => pc += 1,
            InstrKind::Jmp(i) => pc += i,
            InstrKind::Acc(i) => {
                pc += 1;
                acc += i;
            }
        }
        instr.visited = true;
    }

    acc
}

#[aoc(day8, part2)]
pub fn part2_impl1(input: &Vec<Instruction>) -> i64 {
    let mut input = input.clone();
    let mut flipidx = 0;

    while flipidx < input.len() {
        let flip = input.get_mut(flipidx).unwrap();
        let orig = flip.kind;
        match flip.kind {
            InstrKind::Acc(_) => {
                flipidx += 1;
                continue;
            }
            InstrKind::Nop(i) => flip.kind = InstrKind::Jmp(i),
            InstrKind::Jmp(i) => flip.kind = InstrKind::Nop(i),
        }
        drop(flip);

        let mut acc = 0;
        let mut pc = 0i64;
        while (pc as usize) < input.len() && !input[pc as usize].visited {
            let instr = input.get_mut(pc as usize).unwrap();
            match instr.kind {
                InstrKind::Nop(_) => pc += 1,
                InstrKind::Jmp(i) => pc += i,
                InstrKind::Acc(i) => {
                    pc += 1;
                    acc += i;
                }
            }
            instr.visited = true;
        }

        if pc as usize >= input.len() {
            return acc;
        }

        input.get_mut(flipidx).unwrap().kind = orig;
        input.iter_mut().for_each(|i| i.visited = false);
        flipidx += 1;
    }

    unreachable!("no program terminated properly")
}
