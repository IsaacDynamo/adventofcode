use eyre::Result;

use crate::Grid;

type Input = Vec<Grid<char>>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input
        .split("\n\n")
        .map(|block| Grid::new(block.lines().map(|line| line.chars().collect()).collect()))
        .collect())
}

pub fn part1(input: &Input) -> Output {
    let locks = input
        .iter()
        .filter(|grid| {
            grid.iter()
                .filter(|(_, y, _)| *y == 0)
                .all(|(_, _, c)| c == '#')
        })
        .map(|grid| {
            (0..grid.size.0)
                .map(|x| {
                    (0..grid.size.1)
                        .filter(|y| grid.get(x, *y) == Some('#'))
                        .count() as i64
                        - 1
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let keys = input
        .iter()
        .filter(|grid| {
            grid.iter()
                .filter(|(_, y, _)| *y == 6)
                .all(|(_, _, c)| c == '#')
        })
        .map(|grid| {
            (0..grid.size.0)
                .map(|x| {
                    (0..grid.size.1)
                        .filter(|y| grid.get(x, *y) == Some('#'))
                        .count() as i64
                        - 1
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    keys.iter()
        .flat_map(|key| {
            locks
                .iter()
                .filter(|lock| key.iter().zip(lock.iter()).all(|(a, b)| a + b < 6))
        })
        .count()
        .try_into()
        .unwrap()
}

pub fn part2(_input: &Input) -> Output {
    0
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day25/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day25/input.txt")?)?;
    println!("input size {}", input.len());

    assert_eq!(part1(&example), 3);
    println!("part1: {}", part1(&input));
    assert_eq!(part2(&example), 0);
    println!("part2: {}", part2(&input));

    Ok(())
}
