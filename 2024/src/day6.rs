use crate::Grid;
use eyre::Result;
use std::collections::HashSet;

type Input = Grid<char>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(Grid::new(
        input.lines().map(|x| x.chars().collect()).collect(),
    ))
}

pub fn part1(input: &Input) -> Output {
    let start = input.iter().find(|(_, _, c)| *c == '^').unwrap();
    let start = (start.0, start.1);

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut loc = start;
    let mut dir = (0, -1);
    while let Some(c) = input.get(loc.0 + dir.0, loc.1 + dir.1) {
        match c {
            '.' | '^' => {
                loc = (loc.0 + dir.0, loc.1 + dir.1);
                visited.insert(loc);
            }
            '#' => {
                dir = turn(dir);
            }
            _ => unreachable!(),
        }
    }

    visited.len() as _
}

fn cycle(
    input: Grid<char>,
    mut loc: (i64, i64),
    mut dir: (i64, i64),
    mut visited: Grid<u8>,
) -> bool {
    while let Some(c) = input.get(loc.0 + dir.0, loc.1 + dir.1) {
        match c {
            '.' | '^' => {
                loc = (loc.0 + dir.0, loc.1 + dir.1);

                let s = visited.get_mut(loc.0, loc.1).unwrap();

                let m = mask(dir);
                if (*s & m) != 0 {
                    return true;
                } else {
                    *s |= m;
                }
            }
            '#' | 'O' => {
                dir = turn(dir);
            }
            _ => unreachable!(),
        }
    }

    false
}

pub fn part2(input: &Input) -> Output {
    let start = input.iter().find(|(_, _, c)| *c == '^').unwrap();
    let start = (start.0, start.1);

    let mut dir = (0, -1);
    let mut visited = input.map(|_, _, _| 0);
    visited.get_mut(start.0, start.1).map(|x| *x = mask(dir));

    let mut obstruction = HashSet::new();
    let mut loc = start;
    let mut next = (loc.0 + dir.0, loc.1 + dir.1);
    while let Some(c) = input.get(next.0, next.1) {
        match c {
            '.' | '^' => {
                if c == '.' && visited.get(next.0, next.1).unwrap() == 0 {
                    let mut n = input.clone();
                    if let Some(x) = n.get_mut(next.0, next.1) {
                        assert!(*x == '.');
                        *x = 'O';
                        if cycle(n, loc, dir, visited.clone()) {
                            obstruction.insert(next);
                        }
                    }
                }

                loc = (loc.0 + dir.0, loc.1 + dir.1);

                let m = visited.get_mut(loc.0, loc.1).unwrap();
                *m |= mask(dir);
            }
            '#' => {
                dir = turn(dir);
            }
            _ => unreachable!(),
        }
        next = (loc.0 + dir.0, loc.1 + dir.1);
    }

    assert!(!obstruction.contains(&start));

    obstruction.len() as _
}

fn turn(dir: (i64, i64)) -> (i64, i64) {
    (-dir.1, dir.0)
}

fn mask(dir: (i64, i64)) -> u8 {
    match dir {
        (0, 1) => 1,
        (1, 0) => 2,
        (0, -1) => 4,
        (-1, 0) => 8,
        _ => unreachable!(),
    }
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day6/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day6/input.txt")?)?;
    println!("input size {:?}", input.size());

    assert_eq!(part1(&example), 41);
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&example), 6);
    println!("part2: {}", part2(&input));

    Ok(())
}
