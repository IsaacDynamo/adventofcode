use eyre::Result;
use num::{integer::div_rem, Integer};

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

pub fn part1(input: &Input) -> Output {
    input
        .iter()
        .flat_map(|range| {
            (range.0..=range.1).filter_map(|n| {
                let digits = n.ilog10() + 1;
                if digits.is_odd() {
                    return None;
                }
                let q = 10i64.pow(digits / 2);
                let a = n / q;
                let b = n % q;
                (a == b).then_some(n)
            })
        })
        .sum()
}

/// Instead of scaning throught the ranges, which is relativly slow (88ms), it is also possible to generate invalid IDs and see if they are still within the range.
/// This needs some dedup because 2x '111' is equal to 3x '11', but input only has 11 invalid IDs so overhead should be minimal.
pub fn part2(input: &Input) -> Output {
    input
        .iter()
        .flat_map(|range| {
            (range.0..=range.1).filter_map(|n| {
                let digits = n.ilog10() + 1;
                (2..=digits)
                    .filter_map(|chunks| {
                        let size = digits / chunks;
                        (chunks * size == digits).then_some((chunks, size))
                    })
                    .any(|(chunks, size)| {
                        let radix = 10i64.pow(size);
                        let pattern = n % radix;
                        (0..chunks)
                            .scan(n, |s, _| {
                                let (ss, x) = div_rem(*s, radix);
                                *s = ss;
                                Some(x)
                            })
                            .all(|x| x == pattern)
                    })
                    .then_some(n)
            })
        })
        .sum()
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
