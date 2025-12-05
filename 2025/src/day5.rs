use eyre::Result;
use rangemap::RangeInclusiveSet;

type Input = (Vec<(i64, i64)>, Vec<i64>);
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    let mut lines = input.lines();
    let ranges = (&mut lines)
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut i = l.split('-');
            let a = i.next().unwrap().parse().unwrap();
            let b = i.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect();

    let ids = lines.map(|l| l.parse().unwrap()).collect();

    Ok((ranges, ids))
}

pub fn part1(input: &Input) -> Output {
    let ranges = RangeInclusiveSet::from_iter(input.0.iter().map(|(a, b)| *a..=*b));

    input.1.iter().filter(|id| ranges.contains(id)).count() as i64
}

pub fn part2(input: &Input) -> Output {
    let ranges = RangeInclusiveSet::from_iter(input.0.iter().map(|(a, b)| *a..=*b));

    ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("input/day5/example.txt")?)?;
    println!("{:?}", test);
    assert_eq!(part1(&test), 3);

    let input = parse(&read_file("input/day5/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&test), 14);
    println!("part2: {}", part2(&input));

    Ok(())
}
