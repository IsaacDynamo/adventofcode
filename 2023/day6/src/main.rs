use eyre::Result;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::Write;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = (Vec<i64>, Vec<i64>);
type Output = i64;

fn parse(input: &str) -> Result<Input> {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let distance = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    Ok((time, distance))
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("input.txt")?)?;

    assert!(dbg!(part1(&test)) == 288);
    println!("part1: {}", part1(&input));

    assert!(dbg!(part2(&test)) == 71503);
    println!("part2: {}", part2(&input));

    Ok(())
}

fn part1(input: &Input) -> Output {
    input
        .0
        .iter()
        .zip(input.1.iter())
        .map(|(t, d)| (0..*t).into_iter().filter(|i| i * (*t - i) > *d).count() as i64)
        .product()
}

fn part2(input: &Input) -> Output {
    let time = input
        .0
        .iter()
        .fold(String::new(), |mut s, d| {
            s.write_fmt(format_args!("{}", d)).unwrap();
            s
        })
        .parse::<i64>()
        .unwrap();

    let dist = input
        .1
        .iter()
        .fold(String::new(), |mut s, d| {
            s.write_fmt(format_args!("{}", d)).unwrap();
            s
        })
        .parse::<i64>()
        .unwrap();

    part1(&(vec![time], vec![dist]))
}
