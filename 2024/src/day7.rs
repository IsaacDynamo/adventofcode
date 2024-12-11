use eyre::Result;
use num::ToPrimitive;

type Input = Vec<(i64, Vec<i64>)>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| -> Result<_> {
            let mut parts = line.split_terminator(':');
            let r = parts
                .next()
                .ok_or_else(|| eyre::eyre!("bad format"))?
                .parse::<i64>()
                .map_err(eyre::Report::from)?;

            let d = parts
                .next()
                .ok_or_else(|| eyre::eyre!("bad format"))?
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().map_err(eyre::Report::from))
                .collect::<Result<Vec<_>>>()?;

            Ok((r, d))
        })
        .collect()
}

fn search1(acc: i64, digits: &[i64], result: i64) -> bool {
    if digits.is_empty() {
        return acc == result;
    }

    if acc > result {
        return false;
    }

    search1(acc + digits[0], &digits[1..], result) || search1(acc * digits[0], &digits[1..], result)
}

pub fn part1(input: &Input) -> Output {
    input
        .iter()
        .filter_map(|(result, digits)| {
            (search1(digits[0], &digits[1..], *result)).then_some(result)
        })
        .sum()
}

fn search2(acc: i64, numbers: &[i64], result: i64) -> bool {
    if numbers.is_empty() {
        return acc == result;
    }

    if acc > result {
        return false;
    }

    fn concat(a: i64, b: i64) -> i64 {
        let digits = b.to_f64().unwrap().log10().floor() + 1.0;
        let n = 10_f64.powf(digits).to_i64().unwrap();
        a * n + b
    }

    search2(acc + numbers[0], &numbers[1..], result)
        || search2(acc * numbers[0], &numbers[1..], result)
        || search2(concat(acc, numbers[0]), &numbers[1..], result)
}

pub fn part2(input: &Input) -> Output {
    input
        .iter()
        .filter_map(|(result, digits)| {
            (search2(digits[0], &digits[1..], *result)).then_some(result)
        })
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day7/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day7/input.txt")?)?;
    println!("input size {:?}", input.len());

    assert_eq!(part1(&example), 3749);
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&example), 11387);
    println!("part2: {}", part2(&input));

    Ok(())
}
