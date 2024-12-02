use eyre::{Report, Result};

type Input = Vec<Vec<i64>>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().map_err(Report::from))
                .collect::<Result<Vec<_>>>()
        })
        .collect()
}

pub fn part1(input: &Input) -> Output {
    fn safe(levels: &[i64]) -> bool {
        let ascending = levels
            .windows(2)
            .all(|pair| (1..=3).contains(&(pair[1] - pair[0])));
        let descending = || {
            levels
                .windows(2)
                .all(|pair| (1..=3).contains(&(pair[0] - pair[1])))
        };
        ascending || descending()
    }

    input.iter().filter(|levels| safe(levels)).count() as _
}

pub fn part2(input: &Input) -> Output {
    fn all_pairs_skip_one(l: &[i64], pred: fn(i64, i64) -> bool) -> bool {
        fn backtrack_pairs(n: usize, a: i64, b: &[i64], pred: fn(i64, i64) -> bool) -> bool {
            if b.is_empty() {
                true
            } else {
                let normal = pred(a, b[0]) && backtrack_pairs(n, b[0], &b[1..], pred);
                let skip = || n > 0 && backtrack_pairs(n - 1, a, &b[1..], pred);
                normal || skip()
            }
        }

        let normal = backtrack_pairs(1, l[0], &l[1..], pred);
        let skip_first = || backtrack_pairs(0, l[1], &l[2..], pred);
        normal || skip_first()
    }

    fn safe(levels: &[i64]) -> bool {
        let ascending = all_pairs_skip_one(levels, |a, b| (1..=3).contains(&(b - a)));
        let descending = || all_pairs_skip_one(levels, |a, b| (1..=3).contains(&(a - b)));
        ascending || descending()
    }

    input.iter().filter(|levels| safe(levels)).count() as _
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day2/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day2/input.txt")?)?;
    println!("input size {}", input.len());

    assert_eq!(part1(&example), 2);
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&example), 4);
    println!("part2: {}", part2(&input));

    Ok(())
}
