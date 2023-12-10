use eyre::Result;
use std::collections::HashMap;

type Input = Vec<Vec<char>>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input.lines().map(|line| line.chars().collect()).collect())
}

fn get<T: Copy>(grid: &[Vec<T>], x: i64, y: i64) -> Option<T> {
    if x < 0 || y < 0 {
        None
    } else {
        grid.get(y as usize)
            .and_then(|row| row.get(x as usize))
            .copied()
    }
}

static DIRS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn connected(dir: (i64, i64), shape: char) -> Option<(i64, i64)> {
    match dir {
        (0, -1) => "|F!7".match_indices(shape).next(),
        (1, 0) => "J-7!".match_indices(shape).next(),
        (0, 1) => "!L|J".match_indices(shape).next(),
        (-1, 0) => "L!F-".match_indices(shape).next(),
        _ => None,
    }
    .map(|(i, _)| DIRS[i])
}

pub fn part1(input: &Input) -> Output {
    let mut pos = input
        .iter()
        .enumerate()
        .find_map(|(j, line)| {
            line.iter()
                .enumerate()
                .find_map(|(i, c)| (*c == 'S').then_some((i as i64, j as i64)))
        })
        .unwrap();

    let mut dir = DIRS
        .iter()
        .find(|&&dir| {
            let neighbor = get(input, pos.0 + dir.0, pos.1 + dir.1).unwrap_or('.');
            connected(dir, neighbor).is_some()
        })
        .copied()
        .unwrap();

    let mut lenght = 1;
    loop {
        pos = (pos.0 + dir.0, pos.1 + dir.1);
        let neighbor = get(input, pos.0, pos.1).unwrap_or('.');
        if neighbor == 'S' {
            break;
        }
        dir = connected(dir, neighbor).unwrap();
        lenght += 1;
    }

    lenght / 2
}

fn horizontal(a: (i64, i64), b: (i64, i64)) -> i64 {
    match (a, b) {
        (_, (0, 1)) => 1,
        ((0, -1), _) => -1,
        _ => 0,
    }
}

pub fn part2(input: &Input) -> Output {
    let mut pos = input
        .iter()
        .enumerate()
        .find_map(|(j, line)| {
            line.iter()
                .enumerate()
                .find_map(|(i, c)| (*c == 'S').then_some((i as i64, j as i64)))
        })
        .unwrap();

    let mut dir = DIRS
        .iter()
        .find(|&&dir| {
            let neighbor = get(input, pos.0 + dir.0, pos.1 + dir.1).unwrap_or('.');
            connected(dir, neighbor).is_some()
        })
        .copied()
        .unwrap();

    let start_dir = dir;
    let start_pos = pos;
    let mut path = HashMap::<(i64, i64), i64>::new();
    loop {
        pos = (pos.0 + dir.0, pos.1 + dir.1);
        let neighbor = get(input, pos.0, pos.1).unwrap_or('.');
        if neighbor == 'S' {
            break;
        }
        let next = connected(dir, neighbor).unwrap();
        path.insert(pos, horizontal(dir, next));
        dir = next;
    }
    path.insert(start_pos, horizontal(dir, start_dir));
    let path = &path;

    let height = input.len() as i64;
    let width = input[0].len() as i64;
    (0..height)
        .flat_map(|y| {
            (0..width).scan(0, move |level, x| {
                let dy = path.get(&(x, y)).copied();
                if let Some(dy) = dy {
                    *level += dy;
                }
                Some(dy.is_none() && *level != 0)
            })
        })
        .filter(|p| *p)
        .count() as i64
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("day10/test1.txt")?)?;
    println!("{:?}", test);
    assert_eq!(part1(&test), 8);

    let input = parse(&read_file("day10/input.txt")?)?;
    println!("part1: {}", part1(&input));

    let test = parse(&read_file("day10/test2.txt")?)?;
    assert_eq!(part2(&test), 10);

    println!("part2: {}", part2(&input));

    Ok(())
}
