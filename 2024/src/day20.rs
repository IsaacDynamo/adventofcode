use eyre::Result;
use itertools::Itertools;
use std::{cmp::Reverse, collections::BinaryHeap};

use crate::Grid;

type Input = Grid<char>;
type Output = i64;
type Point = (i64, i64);

pub fn parse(input: &str) -> Result<Input> {
    Ok(Grid::from_str(input))
}

fn distance(input: &Grid<bool>, p: Point) -> Grid<Option<i64>> {
    let mut visited = input.map(|_, _, _| None);
    let mut queue = BinaryHeap::new();

    queue.push(Reverse((0, p.0, p.1)));
    while let Some(Reverse((score, x, y))) = queue.pop() {
        if visited.get(x, y) != Some(None) {
            continue;
        }

        if input.get(x, y) != Some(false) {
            continue;
        }

        *(visited.get_mut(x, y).unwrap()) = Some(score);

        queue.push(Reverse((score + 1, x + 1, y)));
        queue.push(Reverse((score + 1, x - 1, y)));
        queue.push(Reverse((score + 1, x, y + 1)));
        queue.push(Reverse((score + 1, x, y - 1)));
    }

    visited
}

fn solve(input: &Input, n: i64, m: i64) -> Output {
    let start = input
        .iter()
        .find_map(|(x, y, c)| (c == 'S').then_some((x, y)))
        .unwrap();
    let end = input
        .iter()
        .find_map(|(x, y, c)| (c == 'E').then_some((x, y)))
        .unwrap();
    let border = input.map(|_, _, c| c == '#');
    let dstart = distance(&border, start);
    let dend = distance(&border, end);
    let d = dstart.get(end.0, end.1).unwrap().unwrap();

    let dend_ref = &dend;
    dstart
        .iter()
        .filter_map(|t| match t {
            (x, y, Some(d)) => Some((x, y, d)),
            _ => None,
        })
        .flat_map(|(x, y, q)| {
            (-n..=n)
                .cartesian_product(-n..=n)
                .filter(|&(dx, dy)| dx.abs() + dy.abs() <= n)
                .filter_map(move |(dx, dy)| {
                    dend_ref.get(x + dx, y + dy).flatten().and_then(|r| {
                        (q + r + dx.abs() + dy.abs() + m <= d).then_some((x, y, dx, dy))
                    })
                })
        })
        .unique()
        .count()
        .try_into()
        .unwrap()
}

pub fn part1(input: &Input) -> Output {
    solve(input, 2, 100)
}

pub fn part2(input: &Input) -> Output {
    solve(input, 20, 100)
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day20/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day20/input.txt")?)?;
    println!("input size {:?} ", input.size());

    assert_eq!(solve(&example, 2, 1), 44);
    println!("part1: {}", part1(&input));
    assert_eq!(solve(&example, 20, 50), 285);
    println!("part2: {}", part2(&input));

    Ok(())
}
