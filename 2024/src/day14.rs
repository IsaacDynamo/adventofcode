use std::collections::HashSet;

use eyre::{Report, Result};
use num::{
    integer::{sqrt, Roots},
    Integer,
};
use regex::{Match, Regex};

type Input = Vec<(Point, Point)>;
type Output = i64;
type Point = (i64, i64);

pub fn parse(input: &str) -> Result<Input> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")?;

    input
        .lines()
        .map(|line| {
            fn n(m: Option<Match>) -> Result<i64> {
                m.ok_or(eyre::eyre!("Nope"))?
                    .as_str()
                    .parse::<i64>()
                    .map_err(Report::from)
            }
            let m = re.captures(line).ok_or(eyre::eyre!("Nope"))?;
            Ok(((n(m.get(1))?, n(m.get(2))?), (n(m.get(3))?, n(m.get(4))?)))
        })
        .collect::<Result<_>>()
}

fn steps(input: &Input, size: Point, secs: i64) -> Output {
    let bots = input
        .iter()
        .map(|&(p, v)| {
            let x = (p.0 + v.0 * secs).mod_floor(&size.0);
            let y = (p.1 + v.1 * secs).mod_floor(&size.1);

            assert!(0 <= x && x < size.0);
            assert!(0 <= y && y < size.1);

            (x, y)
        })
        .collect::<Vec<Point>>();

    let xhalf = size.0 / 2;
    let yhalf = size.1 / 2;

    let quads = bots.iter().fold([0, 0, 0, 0], |mut acc, p| {
        if p.0 < xhalf && p.1 < yhalf {
            acc[0] += 1;
        }
        if p.0 < xhalf && p.1 > yhalf {
            acc[1] += 1;
        }
        if p.0 > xhalf && p.1 < yhalf {
            acc[2] += 1;
        }
        if p.0 > xhalf && p.1 > yhalf {
            acc[3] += 1;
        }

        acc
    });

    quads.iter().product()
}

fn print(bots: &Vec<Point>, size: Point) {
    let set = HashSet::<Point>::from_iter(bots.iter().copied());
    for y in 0..size.1 {
        for x in 0..size.0 {
            if set.contains(&(x, y)) {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!("")
    }
}

pub fn part1(input: &Input) -> Output {
    steps(&input, (101, 103), 100)
}

pub fn part2(input: &Input) -> Output {
    let size = (101, 103);

    for secs in 0.. {
        let bots = input
            .iter()
            .map(|&(p, v)| {
                let x = (p.0 + v.0 * secs).mod_floor(&size.0);
                let y = (p.1 + v.1 * secs).mod_floor(&size.1);

                assert!(0 <= x && x < size.0);
                assert!(0 <= y && y < size.1);

                (x, y)
            })
            .collect::<Vec<Point>>();

        let score = bots
            .iter()
            .map(|p1| {
                bots.iter()
                    .filter(|p2| p1.0 != p2.0 && p1.1 != p2.1)
                    .filter(|p2| p1.0.abs_diff(p2.0) <= 1 && p1.1.abs_diff(p2.1) <= 1)
                    .count() as i64
            })
            .sum::<i64>();

        if score > 500 {
            print(&bots, size);
            return secs;
        }
    }

    unreachable!()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day14/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day14/input.txt")?)?;
    println!("input size {:?}", input.len());

    assert_eq!(steps(&example, (11, 7), 100), 12);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));

    Ok(())
}
