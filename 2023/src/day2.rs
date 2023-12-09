use eyre::Result;
use std::collections::HashMap;

type Input = Vec<(i64, Vec<HashMap<String, i64>>)>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input
        .lines()
        .map(|s| {
            let (game, sets) = s.split_once(':').unwrap();
            let n = game.strip_prefix("Game ").unwrap().parse::<i64>().unwrap();
            let s = sets
                .split(';')
                .map(|elements| {
                    elements
                        .split(',')
                        .map(|element| {
                            let (n, color) = element.trim().split_once(' ').unwrap();
                            (color.to_string(), n.parse().unwrap())
                        })
                        .collect::<HashMap<String, i64>>()
                })
                .collect();
            (n, s)
        })
        .collect())
}

pub fn part1(input: &Input) -> Output {
    let bag = [("red", 12), ("green", 13), ("blue", 14)];
    input
        .iter()
        .filter_map(|game| {
            game.1
                .iter()
                .all(|set| {
                    bag.iter()
                        .all(|(name, n)| set.get(*name).copied().unwrap_or(0) <= *n)
                })
                .then_some(game.0)
        })
        .sum()
}

pub fn part2(input: &Input) -> Output {
    input
        .iter()
        .map(|game| {
            game.1
                .iter()
                .fold(HashMap::<String, i64>::new(), |acc, x| {
                    x.iter().fold(acc, |mut acc, (color, n)| {
                        if let Some(a) = acc.get_mut(color) {
                            *a = (*a).max(*n);
                        } else {
                            acc.insert(color.clone(), *n);
                        }
                        acc
                    })
                })
                .iter()
                .map(|(_, n)| n)
                .product::<i64>()
        })
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("day2/test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("day2/input.txt")?)?;

    assert!(part1(&test) == 8);
    println!("part1: {}", part1(&input));

    assert!(part2(&test) == 2286);
    println!("part2: {}", part2(&input));

    Ok(())
}
