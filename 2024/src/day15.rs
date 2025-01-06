use eyre::{OptionExt, Result};

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

pub fn part2(_input: &Input) -> Output {
    0
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
    // assert_eq!(part2(&example), 45);
    // println!("part2: {}", part2(&input));

    Ok(())
}
