use eyre::Result;

type Input = Vec<Vec<i64>>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap().into()).collect())
        .collect())
}

fn pick(digits: &[i64], lenght: usize, head: i64) -> i64 {
    if lenght == 0 {
        head
    } else {
        let rem = lenght - 1;
        let (offset, x) = digits
            .iter()
            .copied()
            .enumerate()
            .take(digits.len() - rem) // always leave enough digits remaining
            .max_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0).reverse())) // maximum with lowest index
            .unwrap();
        pick(&digits[offset + 1..], rem, 10 * head + x)
    }
}

pub fn part1(input: &Input) -> Output {
    input.iter().map(|l| pick(l, 2, 0)).sum()
}

pub fn part2(input: &Input) -> Output {
    input.iter().map(|l| pick(l, 12, 0)).sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("input/day3/example.txt")?)?;
    println!("{:?}", test);
    assert_eq!(part1(&test), 357);

    let input = parse(&read_file("input/day3/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&test), 3121910778619);
    println!("part2: {}", part2(&input));

    Ok(())
}
