use eyre::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::all_consuming,
    multi::{many, many_m_n, separated_list0},
    sequence::{preceded, terminated},
};

type Input = (Vec<Shape>, Vec<Region>);
type Output = i64;

#[derive(Debug, Clone, Copy)]
pub struct Region {
    a: i64,
    b: i64,
    quantity: [i64; 6],
}

#[derive(Debug, Clone, Copy)]
pub struct Shape {
    #[allow(dead_code)]
    i: i64,
    shape: [[char; 3]; 3],
}

fn number(input: &str) -> IResult<&str, i64> {
    digit1.map(|s: &str| s.parse().unwrap()).parse(input)
}

fn shape(input: &str) -> IResult<&str, Shape> {
    let l = terminated(many_m_n(3, 3, alt((char('#'), char('.')))), char('\n'))
        .map(|l| TryInto::<[char; 3]>::try_into(l).unwrap());
    let s = many_m_n(3, 3, l).map(|x| x.try_into().unwrap());

    (number, tag(":\n"), s)
        .map(|(i, _, shape)| Shape { i, shape })
        .parse(input)
}

fn region(input: &str) -> IResult<&str, Region> {
    let (input, (a, _, b, _)) = (number, char('x'), number, char(':')).parse(input)?;
    let (input, q): (_, Vec<_>) = many(0.., preceded(char(' '), number)).parse(input)?;
    Ok((
        input,
        Region {
            a,
            b,
            quantity: q.try_into().unwrap(),
        },
    ))
}

fn format(input: &str) -> IResult<&str, (Vec<Shape>, Vec<Region>)> {
    let shapes = separated_list0(char('\n'), shape);
    let regions = preceded(char('\n'), separated_list0(char('\n'), region));
    all_consuming((shapes, regions)).parse(input)
}

pub fn parse(input: &str) -> Result<Input> {
    let (_, x) = format(input).unwrap();
    Ok(x)
}

pub fn part1(input: &Input) -> Output {
    let sizes = input
        .0
        .iter()
        .map(|s| {
            s.shape
                .iter()
                .flat_map(|x| x.iter().filter(|x| **x == '#'))
                .count() as i64
        })
        .collect::<Vec<i64>>();

    input
        .1
        .iter()
        .map(|region| {
            let tiles = (region.a / 3) * (region.b / 3);
            let n = region.quantity.iter().sum::<i64>();

            let area = region.a * region.b;
            let min = region
                .quantity
                .iter()
                .zip(sizes.iter())
                .map(|(a, b)| *a * *b)
                .sum::<i64>();

            if n <= tiles {
                1
            } else if area < min {
                0
            } else {
                unimplemented!();
            }
        })
        .sum()
}

pub fn part2(_: &Input) -> Output {
    0
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    println!("{:?}", shape("0:\n###\n##.\n##.\n"));
    println!("{:?}", region("3x4: 4 5 6 6 7 4"));

    let test = parse(&read_file("input/day12/example.txt")?)?;
    println!("{:?}", test);
    //assert_eq!(part1(&test), 2);

    let input = parse(&read_file("input/day12/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&test), 0);
    println!("part2: {}", part2(&input));

    Ok(())
}
