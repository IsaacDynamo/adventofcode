use eyre::Result;
use std::cmp::{max, min};

type Input = (Vec<i64>, Vec<Vec<Vec<i64>>>);
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut maps = vec![];

    _ = lines.next();

    let mut map = vec![];
    for line in lines {
        if line.is_empty() {
            maps.push(map);
            map = vec![];
            continue;
        }
        if line.contains(':') {
            continue;
        }

        let nums = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        map.push(nums);
    }

    maps.push(map);

    Ok((seeds, maps))
}

pub fn part1(input: &Input) -> Output {
    input
        .0
        .iter()
        .map(|s| {
            input.1.iter().fold(*s, |v, m| {
                m.iter()
                    .find(|n| n[1] <= v && v < n[1] + n[2])
                    .map(|n| n[0] + v - n[1])
                    .unwrap_or(v)
            })
        })
        .min()
        .unwrap()
}

pub fn part2(input: &Input) -> Output {
    let ranges = input
        .0
        .chunks_exact(2)
        .map(|pair| (pair[0], pair[0] + pair[1]))
        .collect::<Vec<_>>();

    input
        .1
        .iter()
        .fold(ranges, |ranges, m| {
            let mut new = vec![];
            let rem = m.iter().fold(ranges, |ranges, n| {
                let begin = n[1];
                let end = n[1] + n[2];
                let mut rem = vec![];
                for pair in ranges {
                    if pair.0 < begin {
                        rem.push((pair.0, min(pair.1, begin)));
                    }

                    if pair.1 > end {
                        rem.push((max(end, pair.0), pair.1));
                    }

                    let a = max(pair.0, begin);
                    let b = min(pair.1, end);
                    if b - a > 1 {
                        let shift = n[0] - begin;
                        new.push((a + shift, b + shift));
                    }
                }
                rem
            });
            new.extend(rem.iter());
            new
        })
        .iter()
        .map(|x| x.0)
        .min()
        .unwrap()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("day5/test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("day5/input.txt")?)?;

    assert!(part1(&test) == 35);
    println!("part1: {}", part1(&input));

    assert!(part2(&test) == 46);
    println!("part2: {}", part2(&input));

    Ok(())
}
