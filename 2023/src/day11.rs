use eyre::Result;
use std::cmp::{max, min};

type Input = Vec<Vec<char>>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input.lines().map(|line| line.chars().collect()).collect())
}

pub fn part1(input: &Input) -> Output {
    solve(input, 2)
}

pub fn solve(input: &Input, expander: i64) -> Output {
    let expander = expander - 1;
    let y = input
        .iter()
        .map(|v| v.iter().all(|c| *c == '.') as i64 * expander + 1)
        .collect::<Vec<i64>>();
    let x = input
        .iter()
        .fold(Vec::new(), |mut acc, row| {
            if acc.is_empty() {
                row.iter().map(|c| *c == '.').collect()
            } else {
                for (p, c) in acc.iter_mut().zip(row.iter()) {
                    *p &= *c == '.';
                }
                acc
            }
        })
        .iter()
        .map(|x| *x as i64 * expander + 1)
        .collect::<Vec<_>>();

    let galaxies = input
        .iter()
        .enumerate()
        .flat_map(|(y, v)| {
            v.iter()
                .enumerate()
                .filter_map(move |(x, c)| (*c == '#').then_some((x, y)))
        })
        .collect::<Vec<_>>();

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, q)| {
            galaxies[..i].iter().map(|p| {
                let a = min(p.0, q.0);
                let b = max(p.0, q.0);
                let c = min(p.1, q.1);
                let d = max(p.1, q.1);

                x[a..=b].iter().copied().sum::<i64>() + y[c..=d].iter().copied().sum::<i64>() - 2
            })
        })
        .sum()
}

pub fn part2(input: &Input) -> Output {
    solve(input, 1000000)
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("day11/test.txt")?)?;
    println!("{:?}", test);
    assert_eq!(part1(&test), 374);

    let input = parse(&read_file("day11/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(solve(&test, 10), 1030);
    assert_eq!(solve(&test, 100), 8410);

    println!("part2: {}", part2(&input));

    Ok(())
}
