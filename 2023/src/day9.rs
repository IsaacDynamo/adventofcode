use eyre::Result;

type Input = Vec<Vec<i64>>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect())
}

pub fn part1(input: &Input) -> Output {
    input
        .iter()
        .map(|nums| {
            let mut triangle = vec![nums.clone()];
            loop {
                let delta = triangle
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|x| x[1] - x[0])
                    .collect::<Vec<_>>();
                if delta.iter().all(|x| *x == 0) {
                    break;
                }
                triangle.push(delta);
            }

            triangle
                .iter()
                .rev()
                .map(|v| v.last().unwrap())
                .sum::<i64>()
        })
        .sum()
}

pub fn part2(input: &Input) -> Output {
    input
        .iter()
        .map(|nums| {
            let mut triangle = vec![nums.clone()];
            loop {
                let delta = triangle
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|x| x[1] - x[0])
                    .collect::<Vec<_>>();
                if delta.iter().all(|x| *x == 0) {
                    break;
                }
                triangle.push(delta);
            }

            triangle
                .iter()
                .rev()
                .fold(0, |delta, prevs| prevs.first().unwrap() - delta)
        })
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("day9/test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("day9/input.txt")?)?;

    assert!(part1(&test) == 114);
    println!("part1: {}", part1(&input));

    assert!(part2(&test) == 2);
    println!("part2: {}", part2(&input));

    Ok(())
}
