use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::fmt::{Debug, Display};
use nom::*;
use nom::sequence::terminated;
use nom::character::complete::line_ending;
use nom::character::complete::digit1;
use nom::multi::many_till;
use nom::combinator::{map_res, eof};
use itertools::Itertools;

#[derive(Debug)]
enum AppErr {
    IoError(std::io::Error),
    ParseError(nom::error::ErrorKind, String)
}

impl From<std::io::Error> for AppErr {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl<T: Debug + Display> From<nom::error::Error<T>> for AppErr {
    fn from(err: nom::error::Error<T>) -> Self {
        Self::ParseError(err.code, format!("{:.32}", err.input) )
    }
}

fn parse_line(input: &str) -> IResult<&str, u32> {
    let line = terminated(
        digit1,
        line_ending);
    map_res(line, |val: &str| val.parse::<u32>() )(input)
}

fn parse(input: &str) -> Result<Vec<u32>, AppErr> {
    many_till(parse_line, eof)(input)
        .finish()
        .map(|(_rem,(vec, _eof))| vec )
        .map_err( |e|e.into())
}

fn read_file(path: &str) -> Result<String, AppErr> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<(), AppErr> {

    let test = parse(&read_file("test.txt")?)?;

    assert!(part1( test.iter().copied() ) == 7);

    let input = parse(&read_file("input.txt")? )?;

    println!("{}", part1( input.iter().copied() ));

    assert!(part2(&test) == 5);

    println!("{}", part2(&input));

    Ok(())
}

fn part1(input: impl Iterator<Item = u32>) -> u32 {
    input.tuple_windows::<(_,_)>()
        .filter(|x| x.0 < x.1)
        .count() as u32
}

fn part2(input: & Vec<u32>) -> u32 {
    let x = input
        .windows(3)
        .map(|w| 
            w.iter()
            .copied().sum()
        );

    part1(x)
}
