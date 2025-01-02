use eyre::Result;
use std::ops::Div;

type Input = (i64, i64, i64, Vec<i64>);
type Output = String;

pub fn parse(input: &str) -> Result<Input> {
    let mut lines = input.lines();

    fn num(s: &str) -> i64 {
        s.split(':').nth(1).unwrap().trim().parse::<i64>().unwrap()
    }

    let a = num(lines.next().unwrap());
    let b = num(lines.next().unwrap());
    let c = num(lines.next().unwrap());
    let _ = lines.next();
    let prog = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    Ok((a, b, c, prog))
}

pub fn part1(input: &Input) -> Output {
    let (mut a, mut b, mut c, prog) = input;
    let mut pc = 0;
    let mut output = Vec::new();

    fn combo(imm: i64, a: i64, b: i64, c: i64) -> i64 {
        match imm {
            0..=3 => imm,
            4 => a,
            5 => b,
            6 => c,
            _ => unreachable!(),
        }
    }

    while let Some(&[op, imm]) = prog.get(pc..pc + 2) {
        let mut next_pc = None;
        match op {
            0 => a = a.div(&(1 << combo(imm, a, b, c))),
            1 => b ^= imm,
            2 => b = combo(imm, a, b, c) % 8,
            3 => {
                if a != 0 {
                    next_pc = Some(imm.try_into().unwrap())
                }
            }
            4 => b ^= c,
            5 => output.push(combo(imm, a, b, c) % 8),
            6 => b = a.div(&(1 << combo(imm, a, b, c))),
            7 => c = a.div(&(1 << combo(imm, a, b, c))),
            _ => unreachable!(),
        }

        pc = next_pc.unwrap_or(pc + 2);
    }

    output
        .iter()
        .map(|x| format!("{}", x))
        .reduce(|a, b| format!("{},{}", a, b))
        .unwrap()
}

pub fn part2(input: &Input) -> i64 {
    fn combo(imm: i64, a: i64, b: i64, c: i64) -> i64 {
        match imm {
            0..=3 => imm,
            4 => a,
            5 => b,
            6 => c,
            _ => unreachable!(),
        }
    }

    'iter: for i in 0.. {
        let (_, mut b, mut c, prog) = input;
        let mut pc = 0;
        let mut output = Vec::new();

        let mut a = i;

        while let Some(&[op, imm]) = prog.get(pc..pc + 2) {
            let mut next_pc = None;
            match op {
                0 => a = a.div(&(1 << combo(imm, a, b, c))),
                1 => b ^= imm,
                2 => b = combo(imm, a, b, c) % 8,
                3 => {
                    if a != 0 {
                        next_pc = Some(imm.try_into().unwrap())
                    }
                }
                4 => b ^= c,
                5 => {
                    output.push(combo(imm, a, b, c) % 8);
                    if !prog.starts_with(&output) {
                        continue 'iter;
                    }
                }
                6 => b = a.div(&(1 << combo(imm, a, b, c))),
                7 => c = a.div(&(1 << combo(imm, a, b, c))),
                _ => unreachable!(),
            }

            pc = next_pc.unwrap_or(pc + 2);
        }

        if output == *prog {
            return i;
        }
    }

    unreachable!()
}

fn disas(prog: &[i64]) {
    for instruction in prog.chunks(2) {
        let op = instruction[0];
        let imm = instruction[1];

        fn combo(imm: i64) -> &'static str {
            match imm {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "A",
                5 => "B",
                6 => "C",
                _ => unreachable!(),
            }
        }

        match op {
            0 => println!("{}{} adv 2^{}", op, imm, combo(imm)),
            1 => println!("{}{} bxl {}", op, imm, imm),
            2 => println!("{}{} bst {}", op, imm, combo(imm)),
            3 => println!("{}{} jnz {}", op, imm, imm),
            4 => println!("{}{} bxc", op, imm),
            5 => println!("{}{} out {}", op, imm, combo(imm)),
            6 => println!("{}{} bdv 2^{}", op, imm, combo(imm)),
            7 => println!("{}{} cdv 2^{}", op, imm, combo(imm)),
            _ => unreachable!(),
        }
    }
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day17/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day17/input.txt")?)?;
    println!(
        "input size {} {} {} {}",
        input.0,
        input.1,
        input.2,
        input.3.len()
    );
    println!("{:?}", input.3);
    disas(&input.3);

    assert_eq!(part1(&example), *"4,6,3,5,6,3,5,2,1,0");
    println!("part1: {}", part1(&input));

    let example2 = parse(&read_file("input/day17/example2.txt")?)?;
    println!("{:?}", example2.3);
    disas(&example2.3);
    assert_eq!(part2(&example2), 117440);

    println!("part2: {}", part2(&input));

    Ok(())
}
