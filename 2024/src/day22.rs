use eyre::{Report, Result};
use hashbag::HashBag;
use itertools::Itertools;
use std::collections::HashMap;

type Input = Vec<i64>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| line.parse::<i64>().map_err(Report::from))
        .collect::<Result<Vec<i64>>>()
}

pub fn part1(input: &Input) -> Output {
    input.iter().map(|x| solve(*x)).sum()
}

pub fn part2(input: &Input) -> Output {
    let seq_score: HashBag<(i64, i64, i64, i64)> = input
        .iter()
        .map(|start| {
            (0..=2000)
                .scan(*start, |x, _| {
                    let r = Some(*x);
                    *x = step(*x);
                    r
                })
                .map(|x| x % 10)
                .tuple_windows()
                .map(|(a, b)| (b, b - a))
                .tuple_windows()
                .map(|((_, s1), (_, s2), (_, s3), (cost, s4))| ((s1, s2, s3, s4), cost))
                .fold(HashMap::new(), |mut map, (seq, cost)| {
                    map.entry(seq).or_insert(cost.try_into().unwrap());
                    map
                })
        })
        .map(|map| {
            map.iter().fold(HashBag::new(), |mut bag, (seq, cost)| {
                bag.insert_many(*seq, *cost);
                bag
            })
        })
        .reduce(|mut a, b| {
            a.extend(b.iter());
            a
        })
        .unwrap();

    seq_score
        .set_iter()
        .map(|(_, s)| s)
        .max()
        .unwrap()
        .try_into()
        .unwrap()
}

pub fn step(x: i64) -> i64 {
    fn mix(a: i64, b: i64) -> i64 {
        a ^ b
    }

    fn prune(x: i64) -> i64 {
        x % 16777216
    }

    let s = x;
    let s = prune(mix(s * 64, s));
    let s = prune(mix(s / 32, s));
    prune(mix(s * 2048, s))
}

pub fn solve(x: i64) -> i64 {
    let mut r = x;
    for _ in 0..2000 {
        r = step(r);
    }
    r
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day22/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day22/input.txt")?)?;
    println!("input size {} ", input.len());

    assert_eq!(part1(&example), 37327623);
    println!("part1: {}", part1(&input));

    let example2 = parse(&read_file("input/day22/example2.txt")?)?;
    assert_eq!(part2(&example2), 23);
    println!("part2: {}", part2(&input));

    Ok(())
}
