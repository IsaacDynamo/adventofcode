use eyre::Result;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = (Vec<Vec<char>>, Vec<Vec<usize>>);
type Output = String;

fn parse(input: &str) -> Result<Input> {
    let mut lines = input.lines();

    let mut stack = Vec::new();
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }

        fn hit(i: usize) -> Option<usize> {
            if i < 1 {
                None
            } else {
                ((i-1) % 4 == 0).then_some((i-1)/4)
            }
        }

        for (i,c) in line.chars().enumerate() {
            if let Some(i) = hit(i) {
                if c != ' ' {
                    if stack.len() <= i {
                        stack.resize_with(i + 1, Vec::new);
                    }
                    stack[i].push(c);
                }
            }
        }
    }

    let steps = lines
        .map(|line| {
            line.split_whitespace()
                .enumerate()
                .filter_map(|(i, v)| [1, 3, 5].contains(&i).then(|| v.parse().unwrap()))
                .collect()
        })
        .collect();

    Ok((stack, steps))
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("input.txt")?)?;

    assert!(dbg!(part1(&test)) == "CMZ");
    println!("part1: {}", part1(&input));

    assert!(dbg!(part2(&test)) == "MCD");
    println!("part2: {}", part2(&input));

    Ok(())
}

fn part1(input: &Input) -> Output {

    let (stack, steps) = input;

    let mut stack: Vec<VecDeque<char>> = stack.iter().map(|v| VecDeque::from_iter(v.iter().copied())).collect();

    for step in steps {
        let n = step[0];
        let from = step[1] - 1;
        let to = step[2] - 1;

        for _ in 0..n {
            let v =stack[from].pop_front().unwrap();
            stack[to].push_front(v);
        }

    }

    stack.iter().map(|c| c.front().unwrap()).collect()
}

fn part2(input: &Input) -> Output {
    let (stack, steps) = input;

    let mut stack: Vec<VecDeque<char>> = stack.iter().map(|v| VecDeque::from_iter(v.iter().copied())).collect();

    for step in steps {
        let n = step[0];
        let from = step[1] - 1;
        let to = step[2] - 1;

        let mut group = Vec::new();
        for _ in 0..n {
            group.push(stack[from].pop_front().unwrap());
        }

        for _ in 0..n {
            stack[to].push_front(group.pop().unwrap());
        }
    }

    stack.iter().map(|c| c.front().unwrap()).collect()
}
