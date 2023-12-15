use eyre::Result;

type Input = Vec<Vec<Vec<char>>>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .fold((Vec::new(), true), |(mut acc, new_entry), line| {
            if line.is_empty() {
                (acc, true)
            } else {
                if new_entry {
                    acc.push(Vec::new());
                }
                acc.last_mut().unwrap().push(line);
                (acc, false)
            }
        })
        .0)
}

pub fn part1(input: &Input) -> Output {
    input
        .iter()
        .map(|grid| -> i64 {
            for y in 1..grid.len() {
                if grid[y..]
                    .iter()
                    .zip(grid[..y].iter().rev())
                    .all(|(a, b)| a == b)
                {
                    return y as i64 * 100;
                }
            }

            for x in 1..grid.first().unwrap().len() {
                if grid.iter().all(|line| {
                    line[x..]
                        .iter()
                        .zip(line[..x].iter().rev())
                        .all(|(a, b)| a == b)
                }) {
                    return x as i64;
                }
            }

            0
        })
        .sum()
}

pub fn part2(input: &Input) -> Output {
    input
        .iter()
        .map(|grid| -> i64 {
            for y in 1..grid.len() {
                let diff: i64 = grid[y..]
                    .iter()
                    .zip(grid[..y].iter().rev())
                    .map(|(a, b)| a.iter().zip(b.iter()).filter(|(a, b)| **a != **b).count() as i64)
                    .sum();

                if diff == 1 {
                    return y as i64 * 100;
                }
            }

            for x in 1..grid.first().unwrap().len() {
                let diff: i64 = grid
                    .iter()
                    .map(|line| {
                        line[x..]
                            .iter()
                            .zip(line[..x].iter().rev())
                            .filter(|(a, b)| **a != **b)
                            .count() as i64
                    })
                    .sum();

                if diff == 1 {
                    return x as i64;
                }
            }

            0
        })
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("day13/test.txt")?)?;
    println!("{:?}", test);

    assert_eq!(part1(&test), 405);
    let input = parse(&read_file("day13/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&test), 400);
    println!("part2: {}", part2(&input));

    Ok(())
}
