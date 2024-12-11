use std::collections::{BTreeMap, HashMap, VecDeque};
use eyre::{Report, Result};
use hashbag::HashBag;
use num::{Integer, ToPrimitive};

type Input = Vec<i64>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
        input.split_ascii_whitespace()
            .map(|c| c.to_string().parse::<i64>().map_err(Report::from))
            .collect()
}

fn digits(n: i64) -> usize {
    let digits = n.to_f64().unwrap().log10().floor() + 1.0;
    digits.to_usize().unwrap()
}

fn split_digits(n: i64) -> (i64, i64) {
    let digits = n.to_f64().unwrap().log10().floor() + 1.0;
    let b = 10_f64.powf(digits / 2.0).to_i64().unwrap();
    (n / b, n % b)
}

fn binks(input: &Input, blinks: usize) -> Output {
    let mut stones = input.iter()
        .fold(HashBag::new(), |mut acc, stone| {
            acc.insert(*stone);
            acc
        });

    for _ in 0..blinks {
        let mut next = HashBag::new();
        for (&stone, n) in stones.set_iter() {
            if stone == 0 {
                next.insert_many(1, n);
            } else if digits(stone).is_even() {
                let (a, b) = split_digits(stone);
                next.insert_many(a, n);
                next.insert_many(b, n);
            } else {
                next.insert_many(stone * 2024, n);
            }
        }
        stones = next;
    }

    dbg!(stones.set_len());

    stones.set_iter()
        .map(|(_, n)| n)
        .sum::<usize>()
        .try_into()
        .unwrap()
}

pub fn part1(input: &Input) -> Output {
    binks(input, 25)
}

pub fn part2(input: &Input) -> Output {
    binks(input, 75)
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day11/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day11/input.txt")?)?;
    println!("input size {:?}", input.len());

    assert_eq!(part1(&example), 55312);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));

    Ok(())
}
