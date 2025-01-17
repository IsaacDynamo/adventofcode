use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::fmt::{Debug, Display};
use nom::*;
use nom::character::complete::{line_ending, digit1, space0};
use nom::bytes::complete::{tag};
use nom::multi::{many0, separated_list1};
use nom::combinator::{map_res, eof};
use nom::sequence::{preceded,terminated};
use std::convert::TryFrom;


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

#[derive(Debug)]
struct Card {
    pub fields: Vec<u32>
}

impl TryFrom<Vec<Vec<u32>>> for Card {
    type Error = ();

    fn try_from(value: Vec<Vec<u32>>) -> Result<Self, Self::Error> {

        if value.len() != 5 {
            return Err(())
        }

        for row in &value {
            if row.len() != 5 {
                return Err(())
            }
        }

        Ok(Card {
            fields: value.into_iter().flatten().collect()
        })
    }
}

#[derive(Debug)]
struct Game {
    pub numbers: Vec<u32>,
    pub cards: Vec<Card>
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res( digit1, |n: &str| n.parse::<u32>() )(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    terminated(
        separated_list1(tag(","), parse_number),
        line_ending
    )(input)
}

fn parse_card_row(input: &str) -> IResult<&str, Vec<u32>> {
    terminated(
        separated_list1(tag(" "), preceded(space0, parse_number)),
        line_ending
    )(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let x = many0(
        parse_card_row
    );
    
    map_res(x,  |fields| Card::try_from(fields) )(input)
}

fn parse(input: &str) -> Result<Game, AppErr> {

    let (input, numbers) = parse_numbers(input).finish()?;
    let (input, _) = line_ending::<&str, error::Error<&str>>(input).finish()?;
    let (input, cards) = separated_list1(line_ending, parse_card)(input).finish()?;
    let _ = eof::<&str, error::Error<&str>>(input).finish()?;

    Ok( Game {
        numbers: numbers,
        cards: cards
    })
}

fn read_file(path: &str) -> Result<String, AppErr> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<(), AppErr> {

    let test = parse(&read_file("test.txt")?)?;

    assert!(part1( &test ) == 4512);

    let input = parse(&read_file("input.txt")? )?;

    println!("{}", part1( &input ));

    assert!(part2(&test) == 1924);

    println!("{}", part2(&input));

    Ok(())
}



fn part1(input: &Game) -> u32 {

    let mut state = vec![0u32; input.cards.len()];

    let mask: [u32; 10] = [
        0b00000_00000_00000_00000_11111,
        0b00000_00000_00000_11111_00000,
        0b00000_00000_11111_00000_00000,
        0b00000_11111_00000_00000_00000,
        0b11111_00000_00000_00000_00000,
        0b00001_00001_00001_00001_00001,
        0b00010_00010_00010_00010_00010,
        0b00100_00100_00100_00100_00100,
        0b01000_01000_01000_01000_01000,
        0b10000_10000_10000_10000_10000
    ];

    for &roll in &input.numbers {

        for (i, card) in input.cards.iter().enumerate() {
            for (field, &value) in card.fields.iter().enumerate() {
                if roll == value {
                    state[i] |= 1 << field;
                }
            }

            for &m in mask.iter() {
                if state[i] & m == m {
                    return card.fields.iter().enumerate().filter(|(b, _)| (state[i] & (1 << *b)) == 0 ).fold(0, |sum, (_, &val)| sum + val) * roll;
                }
            }
        }
    }

    panic!("No bingo");
}

#[derive(Default,Copy,Clone)]
struct State {
    done: bool,
    mask: u32
}

fn part2(input: &Game) -> u32 {

    let mut state = vec![State::default(); input.cards.len()];
    let mut remaining = input.cards.len();

    let mask: [u32; 10] = [
        0b00000_00000_00000_00000_11111,
        0b00000_00000_00000_11111_00000,
        0b00000_00000_11111_00000_00000,
        0b00000_11111_00000_00000_00000,
        0b11111_00000_00000_00000_00000,
        0b00001_00001_00001_00001_00001,
        0b00010_00010_00010_00010_00010,
        0b00100_00100_00100_00100_00100,
        0b01000_01000_01000_01000_01000,
        0b10000_10000_10000_10000_10000
    ];

    for &roll in &input.numbers {

        for (i, card) in input.cards.iter().enumerate() {

            if state[i].done {
                continue
            }

            for (field, &value) in card.fields.iter().enumerate() {
                if roll == value {
                    state[i].mask |= 1 << field;
                }
            }

            for &m in mask.iter() {
                if state[i].mask & m == m {

                    assert!(!state[i].done);
                    state[i].done = true;
                    remaining -= 1;

                    if remaining == 0 {
                        return card.fields.iter().enumerate().filter(|(b, _)| (state[i].mask & (1 << *b)) == 0 ).fold(0, |sum, (_, &val)| sum + val) * roll;
                    }

                    break
                }

            }
        }
    }

    panic!("No bingo");
}