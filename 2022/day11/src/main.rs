use eyre::Result;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = Vec<Monkey>;
type Output = i64;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: fn(old: i64) -> i64,
    divisor: i64,
    if_true: usize,
    if_false: usize,
}

fn parse(input: &str) -> Result<Input> {
    let mut monkeys = Vec::new();
    let mut lines = input.lines();

    while let Some(line) = lines.next() {

        assert!(line == format!("Monkey {}:", monkeys.len()), "{:?}", line.chars());

        let items = lines.next().unwrap().split([':', ',']).skip(1).map(|num| num.trim().parse().unwrap()).collect();
        let operation = match lines.next().unwrap().split('=').last().unwrap().trim() {
            "old * 19" => |x| x * 19,
            "old + 6" => |x| x + 6,
            "old * old" => |x| x * x,
            "old + 3" => |x| x + 3,
            "old * 17" => |x| x * 17,
            "old + 1" => |x| x + 1,
            "old + 2" => |x| x + 2,
            "old + 7" => |x| x + 7,
            "old + 8" => |x| x + 8,
            "old * 2" => |x| x * 2,
            x => panic!("No match for: {}", x),
        };

        let divisor = lines.next().unwrap().split_whitespace().last().map(|num| num.parse().unwrap()).unwrap();
        let if_true = lines.next().unwrap().split_whitespace().last().map(|num| num.parse().unwrap()).unwrap();
        let if_false = lines.next().unwrap().split_whitespace().last().map(|num| num.parse().unwrap()).unwrap();

        let _ = lines.next();

        monkeys.push(Monkey {
            items,
            operation,
            divisor,
            if_true,
            if_false
        })
    }

    Ok(monkeys)
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    println!("{:?}", test);
    assert!(dbg!(part1(&test)) == 10605);

    let input = parse(&read_file("input.txt")?)?;
    println!("part1: {:?}", part1(&input));

    assert!(dbg!(part2(&test)) == 2713310158);
    println!("part2: {:?}", part2(&input));

    Ok(())
}

fn part1(input: &Input) -> Output {
    let mut inspected = vec![0; input.len()];
    let mut input = input.clone();

    for _ in 1..=20 {
        for i in 0..input.len() {
            let items = std::mem::take(&mut input[i].items);
            let monkey = input[i].clone();
            for item in items {
                inspected[i] += 1;
                let new = (monkey.operation)(item) / 3;

                if new % monkey.divisor == 0 {
                    input[monkey.if_true].items.push(new);
                } else {
                    input[monkey.if_false].items.push(new);
                }
            }
        }
    }

    inspected.sort();
    inspected.iter().rev().take(2).product()
}

fn part2(input: &Input) -> Output {
    let mut inspected = vec![0; input.len()];
    let mut input = input.clone();

    let modulo: i64 = dbg!(input.iter().map(|m| m.divisor).product());

    for _round in 1..=10000 {
        for i in 0..input.len() {
            let items = std::mem::take(&mut input[i].items);
            let monkey = input[i].clone();
            for item in items {
                inspected[i] += 1;
                let new = (monkey.operation)(item) % modulo;

                if new % monkey.divisor == 0 {
                    input[monkey.if_true].items.push(new);
                } else {
                    input[monkey.if_false].items.push(new);
                }
            }
        }
    }

    inspected.sort();
    inspected.iter().rev().take(2).product()
}
