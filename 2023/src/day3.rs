use eyre::Result;
use std::collections::{HashMap, HashSet};

type Input = Vec<Vec<char>>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input.lines().map(|line| line.chars().collect()).collect())
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

pub fn part1(input: &Input) -> Output {
    let grid = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, c)| {
                (*c != '.' && !c.is_ascii_digit()).then_some(((x as i64, y as i64), *c))
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

pub fn part2(input: &Input) -> Output {
    let gear_loc = input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, c)| (*c == '*').then_some((x as i64, y as i64)))
        })
        .collect::<HashSet<(i64, i64)>>();

    let mut gears = HashMap::<(i64, i64), Vec<i64>>::new();
    for (y, line) in input.iter().enumerate() {
        let mut n: Option<i64> = None;
        let mut gear = HashSet::new();
        for (x, c) in line.iter().enumerate() {
            if let Some(d) = c.to_digit(10) {
                n = Some(n.unwrap_or_default() * 10 + d as i64);
                for &(dx, dy) in NEIGHBOURS.iter() {
                    let p = (x as i64 + dx, y as i64 + dy);
                    if gear_loc.contains(&p) {
                        gear.insert(p);
                    }
                }
            } else {
                assert!(gear.len() <= 1);
                if let Some(p) = gear.iter().next() {
                    gears.entry(*p).or_default().push(n.unwrap())
                }

                gear = HashSet::new();
                n = None;
            }
        }

        assert!(gear.len() <= 1);
        if let Some(p) = gear.iter().next() {
            gears.entry(*p).or_default().push(n.unwrap())
        }
    }

    gears
        .iter()
        .filter_map(|(_, numbers)| (numbers.len() == 2).then(|| numbers.iter().product::<i64>()))
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("day3/test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("day3/input.txt")?)?;

    assert!(part1(&test) == 4361);
    println!("part1: {}", part1(&input));

    assert!(part2(&test) == 467835);
    println!("part2: {}", part2(&input));

    Ok(())
}
