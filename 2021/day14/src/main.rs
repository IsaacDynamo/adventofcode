use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::result::Result;
use std::fmt::{Debug};

#[derive(Debug)]
enum AppErr {
    IoError(std::io::Error),
    IntError(ParseIntError)
}

impl From<std::io::Error> for AppErr {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<ParseIntError> for AppErr {
    fn from(err: ParseIntError) -> Self {
        Self::IntError(err)
    }
}

fn read_file(path: &str) -> Result<String, AppErr> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Rule = (char, char, char);

#[derive(Debug)]
struct Input {
    pub template: String,
    pub rules: Vec<Rule>
}

fn parse(input: &str) -> Result<Input, AppErr> {

    let mut lines = input.lines();

    let template = lines.next().unwrap().to_string();

    lines.next();

    let mut rules = Vec::new();
    while let Some(line) = lines.next() {

        let parts: Vec<_> = line.split(" -> ").collect();

        let pair: Vec<char> = parts[0].chars().collect();

        rules.push( (pair[0], pair[1], parts[1].chars().next().unwrap() ) );
    }

    Ok(Input{template, rules})
}

fn main() -> Result<(), AppErr> {

    let test = parse(&read_file("test.txt")?)?;

    println!("{:?}", test);

    assert!(part1( &test, 10 ) == 1588);

    let input = parse(&read_file("input.txt")? )?;
    println!("{}", part1( &input, 10 ));

    assert!(part1( &test, 40 ) == 2188189693529);
    
    println!("{}", part1( &input, 40 ));

    Ok(())
}


fn part1(input: &Input, k: u32) -> u64 {

    type Pairs = HashMap::<(char, char), u64>;

    let mut pairs = Pairs::new();

    for pair in input.template.chars().collect::<Vec<char>>().windows(2) {
        *pairs.entry( (pair[0], pair[1]) ).or_insert(0) += 1;
    }

    fn step(rules: &Vec<Rule>, pairs: Pairs) -> Pairs {

        let mut result = Pairs::new();

        for rule in rules {

            let n = pairs.get(&(rule.0, rule.1)).copied().unwrap_or(0);

            *result.entry( (rule.0, rule.2) ).or_insert(0) += n;
            *result.entry( (rule.2, rule.1) ).or_insert(0) += n;
        }

        result
    }


    for _ in 0..k {
        pairs = step(&input.rules, pairs);
    }

    let mut elements = HashMap::<char, u64>::new();

    for ((a,b), n) in pairs {

        *elements.entry( a ).or_insert(0) += n;
        *elements.entry( b ).or_insert(0) += n;
    }

    *elements.entry( input.template.chars().next().unwrap() ).or_insert(0) += 1;
    *elements.entry( input.template.chars().rev().next().unwrap() ).or_insert(0) += 1;

    let mut count: Vec<u64> = elements.values().map(|x| x/2 ).collect();

    count.sort();

    count.last().unwrap() - count.first().unwrap()
}