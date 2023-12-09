use eyre::Result;
use std::fmt::Write;

type Input = (Vec<i64>, Vec<i64>);
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let distance = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    Ok((time, distance))
}

pub fn part1(input: &Input) -> Output {
    input
        .0
        .iter()
        .zip(input.1.iter())
        .map(|(t, d)| (0..*t).into_iter().filter(|i| i * (*t - i) > *d).count() as i64)
        .product()
}

pub fn part2(input: &Input) -> Output {
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

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("day6/test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("day6/input.txt")?)?;

    assert!(part1(&test) == 288);
    println!("part1: {}", part1(&input));

    assert!(part2(&test) == 71503);
    println!("part2: {}", part2(&input));

    Ok(())
}
