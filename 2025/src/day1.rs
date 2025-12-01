use eyre::Result;

type Input = Vec<(char, i64)>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input
        .lines()
        .map(|l| {
            let (d, n) = l.split_at(1);
            let d = d.chars().next().unwrap();
            let n = n.parse().unwrap();
            (d, n)
        })
        .collect())
}

fn rotate(i: &(char, i64)) -> i64 {
    match i.0 {
        'L' => -i.1,
        'R' => i.1,
        _ => unreachable!(),
    }
}

pub fn part1(input: &Input) -> Output {
    input
        .iter()
        .scan(50, |s, x| {
            *s += rotate(x);
            *s = s.rem_euclid(100);
            Some(*s)
        })
        .filter(|x| *x == 0)
        .count() as i64
}

pub fn part2(input: &Input) -> Output {
    input
        .iter()
        .scan(50, |s, x| {
            let z = match x.0 {
                'L' => ((100i64 - *s).rem_euclid(100) + x.1) / 100,
                'R' => (*s + x.1) / 100,
                _ => unreachable!(),
            };
            *s += rotate(x);
            *s = s.rem_euclid(100);
            Some(z)
        })
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("input/day1/example.txt")?)?;
    println!("{:?}", test);
    assert_eq!(part1(&test), 3);

    let input = parse(&read_file("input/day1/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&test), 6);
    println!("part2: {}", part2(&input));

    Ok(())
}
