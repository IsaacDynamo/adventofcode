use crate::Grid;
use eyre::{Report, Result};
use std::collections::HashSet;

type Input = Grid<i64>;
type Output = i64;
type Point = (i64, i64);

pub fn parse(input: &str) -> Result<Input> {
    Ok(Grid::new(
        input
            .lines()
            .map(|x| {
                x.chars()
                    .map(|c| c.to_string().parse().map_err(Report::from))
                    .collect::<Result<_>>()
            })
            .collect::<Result<_>>()?,
    ))
}

pub fn part1(input: &Input) -> Output {
    input
        .iter()
        .map(|(x, y, h)| {
            if h == 0 {
                let mut heads = HashSet::new();

                fn step(input: &Input, pos: Point, height: i64, heads: &mut HashSet<Point>) {
                    if let Some(h) = input.get(pos.0, pos.1) {
                        if h == height {
                            if height == 9 {
                                heads.insert(pos);
                            } else {
                                step(input, (pos.0 + 1, pos.1), height + 1, heads);
                                step(input, (pos.0, pos.1 + 1), height + 1, heads);
                                step(input, (pos.0 - 1, pos.1), height + 1, heads);
                                step(input, (pos.0, pos.1 - 1), height + 1, heads);
                            }
                        }
                    }
                }

                step(input, (x, y), 0, &mut heads);

                heads.len().try_into().unwrap()
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &Input) -> Output {
    input
        .iter()
        .map(|(x, y, h)| {
            if h == 0 {
                fn step(input: &Input, pos: Point, height: i64) -> i64 {
                    if let Some(h) = input.get(pos.0, pos.1) {
                        if h == height {
                            if height == 9 {
                                return 1;
                            } else {
                                return step(input, (pos.0 + 1, pos.1), height + 1)
                                    + step(input, (pos.0, pos.1 + 1), height + 1)
                                    + step(input, (pos.0 - 1, pos.1), height + 1)
                                    + step(input, (pos.0, pos.1 - 1), height + 1);
                            }
                        }
                    }

                    0
                }

                step(input, (x, y), 0)
            } else {
                0
            }
        })
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day10/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day10/input.txt")?)?;
    println!("input size {:?}", input.size());

    assert_eq!(part1(&example), 36);
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&example), 81);
    println!("part2: {}", part2(&input));

    Ok(())
}
