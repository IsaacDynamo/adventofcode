use crate::{Grid, DIR};
use eyre::Result;

type Input = Grid<char>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(Grid::new(
        input.lines().map(|x| x.chars().collect()).collect(),
    ))
}

pub fn part1(input: &Input) -> Output {
    input
        .iter()
        .map(|(x, y, c)| {
            if c == 'X' {
                DIR.iter()
                    .filter(|(dx, dy)| {
                        input.get(x + *dx, y + *dy) == Some('M')
                            && input.get(x + (*dx * 2), y + (*dy * 2)) == Some('A')
                            && input.get(x + (*dx * 3), y + (*dy * 3)) == Some('S')
                    })
                    .count() as _
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &Input) -> Output {
    input
        .iter()
        .filter(|(x, y, c)| {
            if *c == 'A' {
                let hit = |(dx, dy): &(i64, i64)| -> bool {
                    input.get(x + *dx, y + *dy) == Some('M')
                        && input.get(x - *dx, y - *dy) == Some('S')
                };
                let forward = [(-1, 1), (1, -1)].iter().any(hit);
                let backward = [(1, 1), (-1, -1)].iter().any(hit);
                forward && backward
            } else {
                false
            }
        })
        .count() as _
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day4/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day4/input.txt")?)?;
    println!("input size {:?}", input.size());

    assert_eq!(part1(&example), 18);
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&example), 9);
    println!("part2: {}", part2(&input));

    Ok(())
}
