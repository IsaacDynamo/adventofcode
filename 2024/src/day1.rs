use std::{collections::HashMap, ops::Mul};

use eyre::{Report, Result};
use num::Integer;

type Input = (Vec<i64>, Vec<i64>);
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    let lines: Vec<Vec<i64>> = input
        .lines()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().map_err(Report::from))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<_>>()?;

    Ok(lines
        .iter()
        .fold((Vec::new(), Vec::new()), |(mut a, mut b), c| {
            a.push(c[0]);
            b.push(c[1]);
            (a, b)
        }))
}

pub fn part1(input: &Input) -> Output {
    let (mut a, mut b) = input.clone();
    a.sort();
    b.sort();
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| a.abs_diff(*b) as i64)
        .sum()
}

pub fn part2(input: &Input) -> Output {
    let (a, b) = input.clone();

    let counts: HashMap<i64, i64> = b.iter().fold(HashMap::<i64, i64>::new(), |mut c, x| {
        if let Some(q) = c.get_mut(x) {
            q.inc();
        } else {
            c.insert(*x, 1);
        }
        c
    });
    a.iter()
        .map(|x| counts.get(x).copied().unwrap_or(0).mul(*x))
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("input/day1/example.txt")?)?;
    println!("{:?}", test);
    assert_eq!(part1(&test), 11);

    let input = parse(&read_file("input/day1/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&test), 31);
    println!("part2: {}", part2(&input));

    Ok(())
}
