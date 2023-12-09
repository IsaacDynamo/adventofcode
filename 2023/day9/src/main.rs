use eyre::Result;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = Vec<Vec<i64>>;
type Output = i64;

fn parse(input: &str) -> Result<Input> {
    Ok(input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect())
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("input.txt")?)?;

    assert!(dbg!(part1(&test)) == 114);
    println!("part1: {}", part1(&input));

    assert!(dbg!(part2(&test)) == 2);
    println!("part2: {}", part2(&input));

    Ok(())
}

fn part1(input: &Input) -> Output {
    input
        .iter()
        .map(|nums| {
            let mut triangle = vec![nums.clone()];
            loop {
                let delta = triangle
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|x| x[1] - x[0])
                    .collect::<Vec<_>>();
                if delta.iter().all(|x| *x == 0) {
                    break;
                }
                triangle.push(delta);
            }

            triangle
                .iter()
                .rev()
                .map(|v| v.last().unwrap())
                .sum::<i64>()
        })
        .sum()
}

fn part2(input: &Input) -> Output {
    input
        .iter()
        .map(|nums| {
            let mut triangle = vec![nums.clone()];
            loop {
                let delta = triangle
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|x| x[1] - x[0])
                    .collect::<Vec<_>>();
                if delta.iter().all(|x| *x == 0) {
                    break;
                }
                triangle.push(delta);
            }

            triangle
                .iter()
                .rev()
                .fold(0, |delta, prevs| prevs.first().unwrap() - delta)
        })
        .sum()
}
