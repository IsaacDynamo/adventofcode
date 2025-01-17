use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::fmt::Debug;
use nom::*;
use nom::sequence::{terminated, separated_pair};
use nom::character::complete::{space1, line_ending, digit1, alpha1};
use nom::multi::many_till;
use nom::combinator::{map_res, eof};


#[derive(Debug)]
enum AppErr {
    IoError(std::io::Error),
    ParseError(String)
}

impl From<std::io::Error> for AppErr {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<nom::error::Error<&str>> for AppErr {
    fn from(err: nom::error::Error<&str>) -> Self {
        Self::ParseError( format!("{}", err) )
    }
}

impl<T: Debug> From<nom::Err<T>> for AppErr {
    fn from(err: nom::Err<T>) -> Self {
        Self::ParseError( format!("{}", err) )
    }
}

fn parse_line(input: &str) -> IResult<&str, Action> {
    let line = terminated(
        separated_pair(alpha1, space1, digit1),
        line_ending);
    map_res(line, |(name, val)| Action::new(name, val) )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Action>> {
    many_till(parse_line, eof)(input).map(|(rem,(vec, _eof))| (rem, vec))
}

fn read_file(path: &str) -> Result<Vec<Action>, AppErr> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    parse(&contents).map(|(_rem, vec)| vec ).map_err(|e| e.into())
}


#[derive(Debug, Copy, Clone)]
enum Action {
    Up(u32),
    Down(u32),
    Forward(u32)
}

impl Action {
    fn new(name: &str, val: &str) -> Result<Self, ()> {

        let val = val.parse::<u32>().map_err(|_|())?;

        Ok(match name {
            "up" => Self::Up(val),
            "down" => Self::Down(val),
            "forward" => Self::Forward(val),
            _ => return Err(())
        })
    }
}


fn main() -> Result<(), AppErr> {

    let test = read_file("test.txt")?;

    assert!(part1( &test ) == 150);

    let input = read_file("input.txt")?;

    println!("{}", part1(&input));

    assert!(part2( &test ) == 900);

    println!("{}", part2(&input));

    Ok(())
}

fn part1( input: &Vec<Action> ) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for &action in input {
        match action {
            Action::Up(v) => depth -= v,
            Action::Down(v) => depth += v,
            Action::Forward(v) => horizontal += v
        }
    }

    horizontal * depth
} 

fn part2( input: &Vec<Action> ) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for &action in input {
        match action {
            Action::Up(v) => aim -= v,
            Action::Down(v) => aim += v,
            Action::Forward(v) => {
                depth += v*aim; 
                horizontal += v;
            }
        }
    }

    horizontal * depth
}