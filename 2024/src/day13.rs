use eyre::{Report, Result};
use num::Integer;
use regex::Regex;

type Input = Vec<(Point, Point, Point)>;
type Output = i64;
type Point = (i64, i64);

pub fn parse(input: &str) -> Result<Input> {
    let a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)")?;
    let b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)")?;
    let r = Regex::new(r"Prize: X=(\d+), Y=(\d+)")?;

    input
        .split("\n\n")
        .map(|block| {
            fn p(re: &Regex, block: &str) -> Result<Point> {
                let m = re.captures(block).ok_or(eyre::eyre!("Nope"))?;
                let x = m
                    .get(1)
                    .ok_or(eyre::eyre!("Nope"))?
                    .as_str()
                    .parse::<i64>()
                    .map_err(Report::from)?;
                let y = m
                    .get(2)
                    .ok_or(eyre::eyre!("Nope"))?
                    .as_str()
                    .parse::<i64>()
                    .map_err(Report::from)?;
                Ok((x, y))
            }

            Ok((p(&a, block)?, p(&b, block)?, p(&r, block)?))
        })
        .collect::<Result<_>>()
}

fn solve(machine: (Point, Point, Point)) -> Option<i64> {
    // AX * a + BX * b = PX
    // AY * a + BY * b = PY

    // a = (PX - BX * b) / AX
    // a = (PY - BY * b) / AY

    // (PX - BX * b) / AX = (PY - BY * b) / AY
    // (PX - BX * b) * AY = (PY - BY * b) * AX
    // PX * AY - BX * b * AY = PY * AX - BY * b * AX
    // BY * b * AX - BX * b * AY = PY * AX - PX * AY
    // b * (BY * AX - BX * AY) = PY * AX - PX * AY
    // b = (PY * AX - PX * AY) / (BY * AX - BX * AY)

    let ((ax, ay), (bx, by), (px, py)) = machine;
    let (b, br) = (py * ax - px * ay).div_rem(&(by * ax - bx * ay));
    let (a, ar) = (px - bx * b).div_rem(&ax);
    (ar == 0 && br == 0).then(|| a * 3 + b)
}

pub fn part1(input: &Input) -> Output {
    input.iter().filter_map(|machine| solve(*machine)).sum()
}

pub fn part2(input: &Input) -> Output {
    input
        .iter()
        .filter_map(|machine| {
            let (a, b, (px, py)) = *machine;
            let c = 10000000000000;
            let p = (px + c, py + c);
            solve((a, b, p))
        })
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day13/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day13/input.txt")?)?;
    println!("input size {:?}", input.len());

    assert_eq!(part1(&example), 480);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));

    Ok(())
}
