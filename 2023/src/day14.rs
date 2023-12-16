use eyre::Result;
use std::collections::HashMap;

type Input = Vec<Vec<char>>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect())
}

fn fixup<T>(grid: &[Vec<T>], dir: Dir, x: i64, y: i64) -> (i64, i64) {
    let hight = grid.len() as i64 - 1;
    let width = grid.first().unwrap().len() as i64 - 1;
    match dir {
        Dir::North => (width - y, x),
        Dir::East => (width - x, hight - y),
        Dir::South => (y, hight - x),
        Dir::West => (x, y),
    }
}

fn size<T>(grid: &[Vec<T>], dir: Dir) -> (i64, i64) {
    let hight = grid.len() as i64;
    let width = grid.first().unwrap().len() as i64;
    match dir {
        Dir::East | Dir::West => (hight, width),
        _ => (width, hight),
    }
}

fn get<T: Copy>(grid: &[Vec<T>], dir: Dir, x: i64, y: i64) -> Option<T> {
    let (x, y) = fixup(grid, dir, x, y);
    if x < 0 || y < 0 {
        None
    } else {
        grid.get(y as usize)
            .and_then(|row| row.get(x as usize))
            .copied()
    }
}

fn get_mut<T>(grid: &mut [Vec<T>], dir: Dir, x: i64, y: i64) -> Option<&mut T> {
    let (x, y) = fixup(grid, dir, x, y);
    if x < 0 || y < 0 {
        None
    } else {
        grid.get_mut(y as usize)
            .and_then(|row| row.get_mut(x as usize))
    }
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    North,
    West,
    South,
    East,
}

fn tilt(grid: &mut [Vec<char>], dir: Dir) {
    let (hight, width) = size(grid, dir);
    for y in 0..hight {
        let mut stop = 0;
        for x in 0..width {
            match get(grid, dir, x, y)
                .ok_or_else(|| format!("{:?} {} {}", dir, x, y))
                .unwrap()
            {
                '#' => stop = x + 1,
                'O' => {
                    *get_mut(grid, dir, x, y).unwrap() = '.';
                    *get_mut(grid, dir, stop, y).unwrap() = 'O';
                    stop += 1;
                }
                _ => (),
            }
        }
    }
}

fn load(grid: &[Vec<char>]) -> i64 {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, line)| (line.iter().filter(|c| **c == 'O').count() * (i + 1)) as i64)
        .sum()
}

pub fn part1(input: &Input) -> Output {
    let mut grid = input.clone();
    tilt(&mut grid, Dir::North);
    load(&grid)
}

pub fn part2(input: &Input) -> Output {
    let mut grid = input.clone();

    let mut history = HashMap::new();
    const N: i64 = 1000000000;
    for i in 1..=N {
        tilt(&mut grid, Dir::North);
        tilt(&mut grid, Dir::West);
        tilt(&mut grid, Dir::South);
        tilt(&mut grid, Dir::East);

        if let Some(prev) = history.get(&grid) {
            let cycle = i - prev;
            let rem = N - i;
            if rem % cycle == 0 {
                break;
            }
        }

        history.insert(grid.clone(), i);
    }

    load(&grid)
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("day14/test.txt")?)?;
    println!("{:?}", test);

    assert_eq!(part1(&test), 136);
    let input = parse(&read_file("day14/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&test), 64);
    println!("part2: {}", part2(&input));

    Ok(())
}
