use std::{cmp::Reverse, collections::BinaryHeap};

use crate::Grid;
use eyre::Result;

type Input = Grid<char>;
type Output = i64;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Point(i64, i64);

impl Point {
    fn add(&self, other: Self) -> Self {
        Point(self.0 + other.0, self.1 + other.1)
    }

    fn sub(&self, other: Self) -> Self {
        Point(self.0 - other.0, self.1 - other.1)
    }

    fn clock(&self) -> Self {
        Point(-self.1, self.0)
    }

    fn counter(&self) -> Self {
        Point(self.1, -self.0)
    }

    fn index(&self) -> usize {
        match (self.0, self.1) {
            (0, 1) => 0,
            (1, 0) => 1,
            (0, -1) => 2,
            (-1, 0) => 3,
            _ => unreachable!(),
        }
    }

    fn dir(i: usize) -> Self {
        match i {
            0 => Self(0, 1),
            1 => Self(1, 0),
            2 => Self(0, -1),
            3 => Self(-1, 0),
            _ => unreachable!(),
        }
    }
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Self(value.0, value.1)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct Cursor(i64, Point, Point);

pub fn parse(input: &str) -> Result<Input> {
    Ok(Grid::from_str(input))
}

pub fn part1(input: &Input) -> Output {
    let s = input
        .iter()
        .find_map(|(x, y, c)| (c == 'S').then_some((x, y).into()))
        .unwrap();
    let dir = (1, 0).into();

    let mut visited = input.map(|_, _, _| [None; 4]);
    let mut queue = BinaryHeap::new();

    queue.push(Reverse(Cursor(0, s, dir)));
    while let Some(Reverse(Cursor(score, pos, dir))) = queue.pop() {
        let cell = input.get(pos.0, pos.1);
        if cell.is_none() || cell == Some('#') {
            continue;
        }

        if cell == Some('E') {
            return score;
        }

        let v = &mut visited.get_mut(pos.0, pos.1).unwrap()[dir.index()];
        if v.is_none() {
            *v = Some(score);
            queue.push(Reverse(Cursor(score + 1, pos.add(dir), dir)));
            queue.push(Reverse(Cursor(score + 1000, pos, dir.clock())));
            queue.push(Reverse(Cursor(score + 1000, pos, dir.counter())));
        } else {
            assert!(v.unwrap() <= score);
        }
    }

    unreachable!()
}

pub fn part2(input: &Input) -> Output {
    let s = input
        .iter()
        .find_map(|(x, y, c)| (c == 'S').then_some((x, y).into()))
        .unwrap();
    let e: Point = input
        .iter()
        .find_map(|(x, y, c)| (c == 'E').then_some((x, y).into()))
        .unwrap();
    let dir = (1, 0).into();

    let mut visited = input.map(|_, _, _| [None; 4]);
    let mut queue = BinaryHeap::new();

    queue.push(Reverse(Cursor(0, s, dir)));
    while let Some(Reverse(Cursor(score, pos, dir))) = queue.pop() {
        let cell = input.get(pos.0, pos.1);
        if cell.is_none() || cell == Some('#') {
            continue;
        }

        let v = &mut visited.get_mut(pos.0, pos.1).unwrap()[dir.index()];
        if v.is_none() {
            *v = Some(score);

            if cell != Some('E') {
                queue.push(Reverse(Cursor(score + 1, pos.add(dir), dir)));
                queue.push(Reverse(Cursor(score + 1000, pos, dir.clock())));
                queue.push(Reverse(Cursor(score + 1000, pos, dir.counter())));
            }
        } else {
            assert!(v.unwrap() <= score);
        }
    }

    let score = visited
        .get(e.0, e.1)
        .unwrap()
        .iter()
        .filter_map(|c| *c)
        .min()
        .unwrap();

    let mut path = input.map(|_, _, _| false);
    let mut work = Vec::new();

    for (i, x) in visited.get(e.0, e.1).unwrap().iter().enumerate() {
        if *x == Some(score) {
            work.push((score, e, Point::dir(i)))
        }
    }

    while let Some((score, pos, dir)) = work.pop() {
        if visited.get(pos.0, pos.1).and_then(|dirs| dirs[dir.index()]) == Some(score) {
            *(path.get_mut(pos.0, pos.1).unwrap()) = true;

            work.push((score - 1, pos.sub(dir), dir));
            work.push((score - 1000, pos, dir.counter()));
            work.push((score - 1000, pos, dir.clock()));
        }
    }

    path.iter()
        .filter(|(_, _, x)| *x)
        .count()
        .try_into()
        .unwrap()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day16/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day16/input.txt")?)?;
    println!("input size {:?}", input.size());

    assert_eq!(part1(&example), 7036);
    println!("part1: {}", part1(&input));
    assert_eq!(part2(&example), 45);
    println!("part2: {}", part2(&input));

    Ok(())
}
