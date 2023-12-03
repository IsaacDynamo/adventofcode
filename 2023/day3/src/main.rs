use eyre::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = Vec<Vec<char>>;
type Output = i64;

fn parse(input: &str) -> Result<Input> {
    Ok(input.lines().map(|line| line.chars().collect()).collect())
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("input.txt")?)?;

    assert!(dbg!(part1(&test)) == 4361);
    println!("part1: {}", part1(&input));

    assert!(dbg!(part2(&test)) == 467835);
    println!("part2: {}", part2(&input));

    Ok(())
}

static NEIGHBOURS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn part1(input: &Input) -> Output {
    let grid = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, c)| {
                (*c != '.' && !c.is_ascii_digit()).then(|| ((x as i64, y as i64), *c))
            })
        })
        .collect::<HashMap<(i64, i64), char>>();

    let mut sum = 0;
    for (y, line) in input.iter().enumerate() {
        let mut n: Option<i64> = None;
        let mut near = false;
        for (x, c) in line.iter().enumerate() {
            if let Some(d) = c.to_digit(10) {
                n = Some(n.unwrap_or_default() * 10 + d as i64);
                near |= NEIGHBOURS
                    .iter()
                    .any(|&(dx, dy)| grid.get(&(x as i64 + dx, y as i64 + dy)).is_some());
            } else {
                if near {
                    sum += n.unwrap_or_default();
                }
                n = None;
                near = false;
            }
        }
    }

    sum
}

fn part2(input: &Input) -> Output {
    let gear_loc = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, c)| (*c == '*').then(|| (x as i64, y as i64)))
        })
        .collect::<HashSet<(i64, i64)>>();

    let mut gears = HashMap::<(i64, i64), Vec<i64>>::new();
    for (y, line) in input.iter().enumerate() {
        let mut n: Option<i64> = None;
        let mut gear: Option<(i64, i64)> = None;
        for (x, c) in line.iter().enumerate() {
            if let Some(d) = c.to_digit(10) {
                n = Some(n.unwrap_or_default() * 10 + d as i64);
                for &(dx, dy) in NEIGHBOURS.iter() {
                    let p = (x as i64 + dx, y as i64 + dy);
                    if gear_loc.contains(&p) {
                        gear = Some(p)
                    }
                }
            } else {
                if let Some(p) = gear {
                    gears.entry(p).or_default().push(n.unwrap())
                }

                gear = None;
                n = None;
            }
        }
    }

    gears
        .iter()
        .filter_map(|(_, numbers)| (numbers.len() == 2).then(|| numbers.iter().product::<i64>()))
        .sum()
}
