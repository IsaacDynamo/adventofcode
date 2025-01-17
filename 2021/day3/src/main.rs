use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::fmt::{Debug, Display};
use nom::*;
use nom::character::complete::{one_of,line_ending};
use nom::multi::many_till;
use nom::combinator::{eof};


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

fn parse_bit(input: &str) -> IResult<&str, u8> {
    one_of("01")(input).map(|(rem, c)| (rem, (c == '1') as u8))
}

type Bits = Vec<u8>;

fn parse_line(input: &str) -> IResult<&str, Bits> {
    many_till(parse_bit, line_ending)(input).map(|(rem,(vec,_term))| (rem, vec))
}

fn parse(input: &str) -> Result<Vec<Bits>, AppErr> {
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

    assert!(part1( &test ) == 198);

    let input = parse(&read_file("input.txt")? )?;

    println!("{}", part1( &input ));

    assert!(part2(&test) == 230);

    println!("{}", part2(&input));

    Ok(())
}

fn bit_sum(input: &Vec<Bits> ) -> Vec<usize> {
    let mut sum = vec![0; input[0].len()];
    for entry in input {
        for (i, &c) in entry.iter().enumerate() {
            sum[i] += c as usize;
        }
    }
    sum
}

fn part1(input: &Vec<Bits>) -> u32 {

    let sum = bit_sum(&input);

    let mut gamma = 0u32;
    let mut epsilon = 0u32;
    let threshold = input.len() / 2;

    for (i,&count) in sum.iter().rev().enumerate() {
        if count > threshold {
            gamma |= 1<<i;
        } else {
            epsilon |= 1<<i;
        }
    }

    gamma * epsilon
}

fn most_common_or_one(count: usize, total: usize) -> u8 {
    (count * 2 >= total) as u8
}

fn least_common_or_zero(count: usize, total: usize) -> u8 {
    (count * 2 < total) as u8
}

fn bits_to_int(bits: &Bits) -> u32 {
    bits.iter()
        .rev()
        .enumerate()
        .fold(0u32, |num, (i,&bit)| num | ((bit as u32) << i) )
}

fn filter(input: &Vec<Vec<u8>>, func: fn (count: usize, total: usize) -> u8 ) -> u32 {
    let width = input[0].len();

    let mut input = (*input).clone();

    for i in 0..width {

        if input.len() == 1 {
            break
        }

        let sum = bit_sum(&input);
        let keep = func(sum[i], input.len());

        input = input.into_iter()
            .filter(|a| a[i] == keep)
            .collect();

    }

    bits_to_int(input[0].as_ref())
}

fn part2(input: &Vec<Bits>) -> u32 {

    let oxy = filter(&input, most_common_or_one);
    let co2 = filter(&input, least_common_or_zero);

    oxy * co2
}
