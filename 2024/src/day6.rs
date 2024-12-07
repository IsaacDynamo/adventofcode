use std::collections::{HashMap, HashSet, VecDeque};

use crate::Grid;
use eyre::Result;

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
                dir = (-dir.1, dir.0);
            }
            _ => unreachable!(),
        }
    }

    visited.len() as _
}

fn cycle(input: Grid<char>, mut loc: (i64, i64),  mut dir: (i64, i64), mut visited: HashMap<(i64, i64), u8> ) -> bool {
    while let Some(c) = input.get(loc.0 + dir.0, loc.1 + dir.1) {
        match c {
            '.' | '^' => {
                loc = (loc.0 + dir.0, loc.1 + dir.1);

                if !visited.contains_key(&loc) {
                    visited.insert(loc, 0);
                }

                let s = visited.get_mut(&loc).unwrap();

                let m = mask(dir);
                if (*s & m) != 0 {
                    return true;
                } else {
                    *s |= m;
                }
            }
            '#' | 'O' => {
                dir = (-dir.1, dir.0);
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

    let mut visited = HashMap::new();
    visited.insert(start, mask(dir));

    let mut obstruction = HashSet::new();

    let mut loc = start;

    let mut next = (loc.0 + dir.0, loc.1 + dir.1);

    while let Some(c) = input.get(next.0, next.1) {
        match c {
            '.' | '^' => {

                if c == '.' && !visited.contains_key(&next) {
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

                if !visited.contains_key(&loc) {
                    visited.insert(loc, 0);
                }

                let m = visited.get_mut(&loc).unwrap();
                *m |= mask(dir);
            }
            '#' => {
                dir = (-dir.1, dir.0);
            }
            _ => unreachable!(),
        }
        next = (loc.0 + dir.0, loc.1 + dir.1);
    }

    assert!(!obstruction.contains(&start));

    obstruction.len() as _
}

pub fn part2a(input: &Input) -> Output {
    let start = input.iter().find(|(_, _, c)| *c == '^').unwrap();
    let start = (start.0, start.1);

    let mut dir = (0, -1);
    let mut obstruction = HashSet::new();

    let (m, n) = input.size();
    for i in 0..m {
        for j in 0..n {
            if input.get(i, j) == Some('.') {
                let mut n = input.clone();
                let q = n.get_mut(i,j).unwrap();
                *q = 'O';
                if cycle(n, start, dir, HashMap::new()) {
                    obstruction.insert((i, j));
                }
            }
        }
    }

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
        _ => unreachable!()
    }
}

fn part2_bad(input: &Input) -> Output {
    let start = input.iter().find(|(_, _, c)| *c == '^').unwrap();
    let start = (start.0, start.1);
    let start_dir = (0, -1);


    let mut visited = HashMap::new();
    visited.insert(start, HashMap::from_iter([(start_dir, 0)].into_iter()));

    //let mut obstruction = HashSet::new();


    let mut dir = start_dir;
    let mut loc = start;
    let mut i = 0;
    while let Some(c) = input.get(loc.0 + dir.0, loc.1 + dir.1) {
        match c {
            '.' | '^' => {
                loc = (loc.0 + dir.0, loc.1 + dir.1);

                if !visited.contains_key(&loc) {
                    visited.insert(loc, HashMap::new());
                }

                let m = visited.get_mut(&loc).unwrap();
                let old = m.insert(dir, i);
                assert!(old.is_none());
                i += 1;

            }
            '#' | 'O' => {
                dir = (-dir.1, dir.0);
            }
            _ => unreachable!(),
        }
    }


    let mut cursors = VecDeque::new();
    let dir = start_dir;
    let loc = start;
    let i = 0;
    cursors.push_back((loc, dir, i));

    while let Some((loc, dir, i)) = cursors.pop_front() {
        match input.get(loc.0, loc.1) {
            Some('.') => {

                let prev = (loc.0 - dir.0, loc.1 - dir.1);

                cursors.push_back((prev, dir, i-1));

            }
            _ => (),
        }
    }

    // while let Some(c) = input.get(loc.0 - dir.0, loc.1 - dir.1) {
    //     match c {
    //         '.' | '^' => {
    //             loc = (loc.0 - dir.0, loc.1 - dir.1);

    //             if !visited.contains_key(&loc) {
    //                 visited.insert(loc, HashMap::new());
    //             }

    //             let m = visited.get_mut(&loc).unwrap();
    //             i += 1;
    //             m.insert(dir, i);

    //         }
    //         '#' | 'O' => {
    //             dir = (-dir.1, dir.0);
    //         }
    //         _ => unreachable!(),
    //     }
    // }

    todo!()

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

    let p2 = part2(&input);
    println!("part2: {}", p2);
    assert_eq!(p2 , 1915);

    Ok(())
}
