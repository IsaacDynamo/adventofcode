use eyre::{Report, Result};

type Input = Vec<i64>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(vec![])
}

pub fn part1(input: &Input) -> Output {
    0
}

pub fn part2(input: &Input) -> Output {
    0
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("input/day2/example.txt")?)?;
    println!("{:?}", test);
    assert_eq!(part1(&test), -1);

    let input = parse(&read_file("input/day2/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&test), -1);
    println!("part2: {}", part2(&input));

    Ok(())
}
