use eyre::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = Vec<(i64, Vec<HashMap<String, i64>>)>;
type Output = i64;

fn parse(input: &str) -> Result<Input> {
    Ok(input
        .lines()
        .map(|s| {
            let (game, sets) = s.split_once(":").unwrap();
            let n = game.strip_prefix("Game ").unwrap().parse::<i64>().unwrap();
            let s = sets
                .split(";")
                .map(|elements| {
                    elements
                        .split(",")
                        .map(|element| {
                            let (n, color) = element.trim().split_once(" ").unwrap();
                            (color.to_string(), n.parse().unwrap())
                        })
                        .collect::<HashMap<String, i64>>()
                })
                .collect();
            (n, s)
        })
        .collect())
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("input.txt")?)?;

    assert!(dbg!(part1(&test)) == 8);
    println!("part1: {}", part1(&input));

    assert!(dbg!(part2(&test)) == 2286);
    println!("part2: {}", part2(&input));

    Ok(())
}

fn part1(input: &Input) -> Output {
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
                .then(|| game.0)
        })
        .sum()
}

fn part2(input: &Input) -> Output {
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
