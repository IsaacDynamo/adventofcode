use eyre::{OptionExt, Result};
use std::{collections::HashSet, fmt::Write};

use crate::Grid;

type Input = (Grid<char>, Vec<char>);
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    let mut parts = input.split("\n\n");

    let grid = Grid::from_str(parts.next().ok_or_eyre("no grid")?);
    let moves = parts
        .next()
        .ok_or_eyre("no grid")?
        .chars()
        .filter(|c| *c != '\n')
        .collect();

    Ok((grid, moves))
}

pub fn part1(input: &Input) -> Output {
    let (grid, moves) = input;

    let mut grid = grid.clone();
    let mut pos = grid
        .iter()
        .find_map(|(x, y, c)| (c == '@').then_some((x, y)))
        .unwrap();
    (*grid.get_mut(pos.0, pos.1).unwrap()) = '.';

    for m in moves {
        let dir = match *m {
            '>' => (1, 0),
            '<' => (-1, 0),
            'v' => (0, 1),
            '^' => (0, -1),
            _ => unreachable!(),
        };

        let next = (pos.0 + dir.0, pos.1 + dir.1);
        let dest = grid.get(next.0, next.1).unwrap();
        match dest {
            '.' => pos = next,
            '#' => (),
            'O' => {
                let mut scan = (next.0 + dir.0, next.1 + dir.1);
                let mut c = grid.get(scan.0, scan.1).unwrap();

                while c == 'O' {
                    scan = (scan.0 + dir.0, scan.1 + dir.1);
                    c = grid.get(scan.0, scan.1).unwrap();
                }

                if c == '.' {
                    (*grid.get_mut(next.0, next.1).unwrap()) = '.';
                    (*grid.get_mut(scan.0, scan.1).unwrap()) = 'O';
                    pos = next;
                }
            }
            _ => unreachable!(),
        }
    }

    grid.iter()
        .map(|(x, y, c)| if c == 'O' { y * 100 + x } else { 0 })
        .sum()
}

pub fn part2(input: &Input) -> Output {
    let (grid, moves) = input;

    let mut grid = Grid::from_str(
        grid.iter()
            .fold(String::new(), |mut acc, (x, _, c)| {
                let s = match c {
                    '#' => "##",
                    '.' => "..",
                    'O' => "[]",
                    '@' => "@.",
                    _ => unreachable!(),
                };

                acc.write_str(s).unwrap();

                if x + 1 == grid.size().0 {
                    acc.write_char('\n').unwrap();
                }

                acc
            })
            .as_str(),
    );

    let mut pos = grid
        .iter()
        .find_map(|(x, y, c)| (c == '@').then_some((x, y)))
        .unwrap();
    (*grid.get_mut(pos.0, pos.1).unwrap()) = '.';

    for m in moves {
        let dir = match *m {
            '>' => (1, 0),
            '<' => (-1, 0),
            'v' => (0, 1),
            '^' => (0, -1),
            _ => unreachable!(),
        };

        let next = (pos.0 + dir.0, pos.1 + dir.1);
        let dest = grid.get(next.0, next.1).unwrap();
        match dest {
            '.' => pos = next,
            '#' => (),
            '[' | ']' => {
                fn moving(
                    grid: &Grid<char>,
                    pieces: &mut HashSet<(i64, i64, char)>,
                    dir: (i64, i64),
                    pos: (i64, i64),
                ) -> bool {
                    let c = grid.get(pos.0, pos.1).unwrap();
                    let (other, delta) = match c {
                        '.' => return true,
                        '#' => return false,
                        '[' => (']', 1),
                        ']' => ('[', -1),
                        _ => unreachable!(),
                    };

                    if dir.0 != 0 {
                        pieces.insert((pos.0, pos.1, c));
                        pieces.insert((pos.0 + dir.0, pos.1, other));
                        let n = (pos.0 + 2 * dir.0, pos.1);
                        moving(grid, pieces, dir, n)
                    } else {
                        pieces.insert((pos.0, pos.1, c));
                        pieces.insert((pos.0 + delta, pos.1, other));
                        moving(grid, pieces, dir, (pos.0, pos.1 + dir.1))
                            && moving(grid, pieces, dir, (pos.0 + delta, pos.1 + dir.1))
                    }
                }

                let mut pieces = HashSet::new();
                if moving(&grid, &mut pieces, dir, next) {
                    for (x, y, _) in &pieces {
                        *(grid.get_mut(*x, *y).unwrap()) = '.';
                    }

                    for (x, y, c) in &pieces {
                        *(grid.get_mut(*x + dir.0, *y + dir.1).unwrap()) = *c;
                    }

                    pos = next;
                }
            }
            _ => unreachable!(),
        }
    }

    grid.iter()
        .map(|(x, y, c)| if c == '[' { y * 100 + x } else { 0 })
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day15/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day15/input.txt")?)?;
    println!("input size {:?} {}", input.0.size(), input.1.len());

    assert_eq!(part1(&example), 10092);
    println!("part1: {}", part1(&input));
    assert_eq!(part2(&example), 9021);
    println!("part2: {}", part2(&input));

    Ok(())
}
