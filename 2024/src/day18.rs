use eyre::{Report, Result};
use std::{cmp::Reverse, collections::BinaryHeap};

use crate::Grid;

type Input = Vec<(i64, i64)>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<i64>().map_err(Report::from))
                .collect::<Result<Vec<_>>>()
                .map(|n| (n[0], n[1]))
        })
        .collect::<Result<Vec<(i64, i64)>>>()
}

fn p(input: &[(i64, i64)], size: usize) -> Output {
    let mut grid = Grid::new(vec![vec![false; size + 1]; size + 1]);
    for &(x, y) in input {
        *(grid.get_mut(x, y).unwrap()) = true;
    }
    assert!(grid.get(0, 0) == Some(false));

    let mut visited = grid.map(|_, _, _| None);
    let mut queue = BinaryHeap::new();

    queue.push(Reverse((0, 0, 0)));
    while let Some(Reverse((score, x, y))) = queue.pop() {
        if visited.get(x, y) != Some(None) {
            continue;
        }

        if grid.get(x, y) != Some(false) {
            continue;
        }

        *(visited.get_mut(x, y).unwrap()) = Some(score);

        queue.push(Reverse((score + 1, x + 1, y)));
        queue.push(Reverse((score + 1, x - 1, y)));
        queue.push(Reverse((score + 1, x, y + 1)));
        queue.push(Reverse((score + 1, x, y - 1)));
    }

    visited.get(size as i64, size as i64).unwrap().unwrap()
}

pub fn part1(input: &Input) -> Output {
    p(&input[..1024], 70)
}

fn p2(input: &[(i64, i64)], size: usize) -> String {
    for n in 1..input.len() {
        let mut grid = Grid::new(vec![vec![false; size + 1]; size + 1]);
        for &(x, y) in &input[0..=n] {
            *(grid.get_mut(x, y).unwrap()) = true;
        }
        assert!(grid.get(0, 0) == Some(false));

        let mut visited = grid.map(|_, _, _| None);
        let mut queue = BinaryHeap::new();

        queue.push(Reverse((0, 0, 0)));
        while let Some(Reverse((score, x, y))) = queue.pop() {
            if visited.get(x, y) != Some(None) {
                continue;
            }

            if grid.get(x, y) != Some(false) {
                continue;
            }

            *(visited.get_mut(x, y).unwrap()) = Some(score);

            queue.push(Reverse((score + 1, x + 1, y)));
            queue.push(Reverse((score + 1, x - 1, y)));
            queue.push(Reverse((score + 1, x, y + 1)));
            queue.push(Reverse((score + 1, x, y - 1)));
        }

        if visited.get(size as i64, size as i64).unwrap().is_none() {
            return format!("{},{}", input[n].0, input[n].1);
        }
    }

    unreachable!()
}

pub fn part2(input: &Input) -> String {
    p2(input, 70)
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day18/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day18/input.txt")?)?;
    println!("input size {} ", input.len());

    assert_eq!(p(&example[..12], 6), 22);
    println!("part1: {}", part1(&input));
    assert_eq!(p2(&example, 6), *"6,1");
    println!("part2: {}", part2(&input));

    Ok(())
}
