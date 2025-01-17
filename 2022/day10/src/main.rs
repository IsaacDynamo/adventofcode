use eyre::Result;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = Vec<Instr>;
type Output = i32;

#[derive(Debug)]
enum Instr {
    Noop,
    Addx(i32),
}

fn parse(input: &str) -> Result<Input> {
    Ok(input.lines().map(|l| {
        let mut parts = l.split_whitespace();
        match parts.next().unwrap() {
            "noop" => Instr::Noop,
            "addx" => Instr::Addx(parts.next().unwrap().parse().unwrap()),
            _ => panic!(),
        }
    }).collect())
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    println!("{:?}", test);
    assert!(dbg!(part1(&test)) == 13140);

    let input = parse(&read_file("input.txt")?)?;
    println!("part1: {:?}", part1(&input));

    part2(&test);
    part2(&input);

    Ok(())
}

fn part1(input: &Input) -> Output {
    let mut cycle = 0;
    let mut reg_x = 1;
    let mut sum = 0;

    fn tick(cycle: &mut i32, sum: &mut i32, x: i32) {
        *cycle += 1;
        if *cycle >= 20 && (*cycle - 20) % 40 == 0 {
            *sum += x * *cycle;
        }
    }

    for instr in input {
        match instr {
            Instr::Noop => {
                tick(&mut cycle, &mut sum, reg_x);
            },
            &Instr::Addx(v) => {
                tick(&mut cycle, &mut sum, reg_x);
                tick(&mut cycle, &mut sum, reg_x);
                reg_x += v;
            },
        }

        if cycle > 220 {
            break
        }
    }

    sum
}

fn part2(input: &Input) {
    let mut cycle = 0;
    let mut reg_x = 1;

    fn tick(cycle: &mut i32, x: i32) {
        *cycle += 1;

        let col = *cycle % 40;
        let d = col - x;
        if 0 <= d && d <= 2 {
            print!("#");
        } else {
            print!(" ")
        }

        if (*cycle - 1) % 40 == 39 {
            println!("");
        }
    }

    for instr in input {
        match instr {
            Instr::Noop => {
                tick(&mut cycle, reg_x);
            },
            &Instr::Addx(v) => {
                tick(&mut cycle, reg_x);
                tick(&mut cycle, reg_x);
                reg_x += v;
            },
        }

        if cycle > 240 {
            break
        }
    }
}
