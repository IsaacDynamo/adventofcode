use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::fmt::{Debug};

#[derive(Debug)]
enum AppErr {
    IoError(std::io::Error),
}

impl From<std::io::Error> for AppErr {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

fn read_file(path: &str) -> Result<String, AppErr> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


fn parse(input: &str) -> Result<Vec<String>, AppErr> {

    Ok(input.lines()
        .map( |line| line.to_string() )
        .collect())
}

fn main() -> Result<(), AppErr> {

    let test = parse(&read_file("test.txt")?)?;

    assert!(part1( &test ) == 26397);

    let input = parse(&read_file("input.txt")? )?;

    println!("{}", part1( &input ));

    assert!(part2(&test ) == 288957);
 
    println!("{}", part2(&input));

    Ok(())
}

fn part1( input: &Vec<String>) -> u32 {

    fn points(c: char) -> u32 {
        match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
         _  => panic!()
        }
    }
    
    let mut score = 0;

    for line in input {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '[' | '<' | '(' | '{' => stack.push(c),
                ']' => {
                    if stack.pop() != Some('[') {
                        score += points(c);
                        break
                    }
                },
                '>' => {
                    if stack.pop() != Some('<') {
                        score += points(c);
                        break
                    }
                },
                ')' => {
                    if stack.pop() != Some('(') {
                        score += points(c);
                        break
                    }
                },
                '}' => {
                    if stack.pop() != Some('{') {
                        score += points(c);
                        break
                    }
                },
                _ => panic!()
            }

        }
    }

    score
}


fn part2( input: &Vec<String>) -> u64 {

    fn points(c: char) -> u64 {
        match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
         _  => panic!()
        }
    }
    
    let mut scores = Vec::new();

    'skip: for line in input {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '[' | '<' | '(' | '{' => stack.push(c),
                ']' => {
                    if stack.pop() != Some('[') {
                        continue 'skip
                    }
                },
                '>' => {
                    if stack.pop() != Some('<') {
                        continue 'skip
                    }
                },
                ')' => {
                    if stack.pop() != Some('(') {
                        continue 'skip
                    }
                },
                '}' => {
                    if stack.pop() != Some('{') {
                        continue 'skip
                    }
                },
                _ => panic!()
            }
        }

        let mut score = 0u64;
        for &c in stack.iter().rev() {
            score = score * 5 + points(c);
        }
    
        scores.push( score );
    }

    assert!(scores.len() % 2 == 1);

    scores.sort();

    scores[scores.len() / 2]

}