use crate::grid::Grid;
use eyre::Result;

type Input = (Vec<Vec<i64>>, Vec<char>, Grid<char>);
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    let lines = input.lines().count();
    let nums: Vec<Vec<i64>> = input
        .lines()
        .take(lines - 1)
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    let ops: Vec<char> = input
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|c| c.chars().next().unwrap())
        .collect();

    let grid = Grid::from_str(input);

    Ok((nums, ops, grid))
}

pub fn part1(input: &Input) -> Output {
    let (nums, ops, _) = input;
    ops.iter()
        .enumerate()
        .map(|(i, c)| -> i64 {
            match c {
                '+' => nums.iter().map(|v| v[i]).sum(),
                '*' => nums.iter().map(|v| v[i]).product(),
                _ => unreachable!(),
            }
        })
        .sum()
}

pub fn part2(input: &Input) -> Output {
    let (_, _, grid) = input;
    let (w, h) = grid.size();
    (0..w)
        .rev()
        .fold((0, 0, 1), |(total, sum, prod), x| {
            let ops = grid.get(x, h - 1).unwrap();
            let n = (0..h - 1)
                .rev()
                .map(|y| grid.get(x, y).unwrap())
                .filter_map(|c| c.to_digit(10).map(|x| x as i64))
                .scan(1, |s, x| {
                    let m = *s;
                    *s *= 10;
                    Some(x * m)
                })
                .reduce(|a, b| a + b);

            match (ops, n) {
                ('+', Some(n)) => (total + sum + n, 0, 1),
                ('*', Some(n)) => (total + prod * n, 0, 1),
                (' ', Some(n)) => (total, sum + n, prod * n),
                _ => (total, 0, 1),
            }
        })
        .0
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("input/day6/example.txt")?)?;
    println!("{:?}", test);
    assert_eq!(part1(&test), 4277556);

    let input = parse(&read_file("input/day6/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&test), 3263827);
    println!("part2: {}", part2(&input));

    Ok(())
}
