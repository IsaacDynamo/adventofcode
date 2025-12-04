use eyre::Result;
use std::collections::HashSet;

type Input = Vec<(i64, i64)>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input
        .split(',')
        .map(|p| {
            let mut i = p.split('-');
            let a = i.next().unwrap().parse().unwrap();
            let b = i.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect())
}

fn digits(n: i64) -> i64 {
    (n.ilog10() + 1) as i64
}

fn pow10(n: i64) -> i64 {
    10i64.pow(n as u32)
}

struct Range {
    start: i64,
    stop: i64,
}

impl Range {
    fn new(start: i64, stop: i64) -> Self {
        Self { start, stop }
    }
}

impl Iterator for Range {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.stop {
            None
        } else {
            let len = digits(self.start);
            let x = pow10(len);
            let a = self.start;
            let b = self.stop.min(x - 1);

            self.start = x;

            Some((a, b))
        }
    }
}

fn normalize(input: &Input) -> impl Iterator<Item = (i64, i64)> {
    input.iter().flat_map(|(a, b)| Range::new(*a, *b))
}

fn chunks(digits: i64) -> impl Iterator<Item = (i64, i64)> {
    (2..=digits).filter_map(move |n| {
        let size = digits / n;
        (n * size == digits).then_some((n, size))
    })
}

pub fn part1(input: &Input) -> Output {
    let mut invalid = HashSet::new();

    normalize(input).for_each(|(a, b)| {
        let d = digits(a);
        let size = d / 2;

        if 2 * size == d {
            let q = pow10(size);
            let qqq = q + 1;
            let a_head = a / q;
            let b_head = b / q;

            (a_head..=b_head).for_each(|p| {
                let id = p * qqq;
                if a <= id && id <= b {
                    invalid.insert(id);
                }
            });
        }
    });

    invalid.iter().sum()
}

pub fn part2(input: &Input) -> Output {
    let mut invalid = HashSet::new();

    normalize(input).for_each(|(a, b)| {
        chunks(digits(a)).for_each(|(n, size)| {
            let q = pow10((n - 1) * size);
            let qqq: i64 = (0..n).map(|x| pow10(x * size)).sum();
            let a_head = a / q;
            let b_head = b / q;

            (a_head..=b_head).for_each(|p| {
                let id = p * qqq;
                if a <= id && id <= b {
                    invalid.insert(id);
                }
            });
        })
    });

    invalid.iter().sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("input/day2/example.txt")?)?;
    println!("{:?}", test);
    assert_eq!(part1(&test), 1227775554);

    let input = parse(&read_file("input/day2/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&test), 4174379265);
    println!("part2: {}", part2(&input));

    Ok(())
}
