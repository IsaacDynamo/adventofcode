use eyre::Result;
use std::collections::VecDeque;

use crate::grid::{DIR, Grid};

type Input = Grid<char>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(Grid::from_str(input))
}

fn neighbours(grid: &Grid<char>, x: i64, y: i64) -> i64 {
    DIR.iter()
        .filter_map(|(dx, dy)| grid.get(x + dx, y + dy))
        .filter(|c| *c == '@')
        .count() as i64
}

pub fn part1(input: &Input) -> Output {
    input
        .iter()
        .filter(|(_, _, c)| *c == '@')
        .map(|(x, y, _)| neighbours(input, x, y))
        .filter(|x| *x < 4)
        .count() as i64
}

pub fn part2(input: &Input) -> Output {
    let mut remove = VecDeque::new();
    let mut remaining = input.map(|x, y, c| {
        if c == '@' {
            let n = neighbours(input, x, y);
            if n < 4 {
                remove.push_back((x, y));
            }
            n
        } else {
            -1
        }
    });

    let mut removed = 0;
    while let Some((x, y)) = remove.pop_front() {
        removed += 1;
        DIR.iter().for_each(|(dx, dy)| {
            if let Some(r) = remaining.get_mut(x + dx, y + dy)
                && *r >= 4
            {
                *r -= 1;
                if *r == 3 {
                    remove.push_back((x + dx, y + dy));
                }
            }
        });
    }

    removed
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("input/day4/example.txt")?)?;
    println!("{:?}", test);
    assert_eq!(part1(&test), 13);

    let input = parse(&read_file("input/day4/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&test), 43);
    println!("part2: {}", part2(&input));

    Ok(())
}
