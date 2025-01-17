use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::fmt::{Debug};
use std::collections::hash_map::HashMap;

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


fn parse(input: &str) -> Result<Vec<Vec<u32>>, AppErr> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push( 
            line.split(" -> ")
            .map(|cord| cord.split(","))
            .flatten()
            .map(|x| x.parse::<u32>().unwrap())
            .collect()
        );
    }
    Ok(result)
}

fn main() -> Result<(), AppErr> {

    let test = parse(&read_file("test.txt")?)?;

    assert!(part1( &test ) == 5);

    let input = parse(&read_file("input.txt")? )?;

    println!("{}", part1( &input ));

    assert!(part2(&test) == 12);

    println!("{}", part2(&input));

    Ok(())
}

fn from_to(a: u32, b: u32) -> Box<dyn Iterator<Item = u32>> {
    if a < b {
        Box::new( a..=b )
    } else {
        Box::new( (b..=a).rev() )
    }
}


fn part1(cords: &Vec<Vec<u32>>) -> u32 {

    let mut diagram = HashMap::<(u32,u32), u32>::new();

    for line in cords {
        if !(line[0] == line[2] || line[1] == line[3]) {
            continue
        }

        for x in from_to(line[0], line[2]) {
            for y in from_to(line[1], line[3]) {
                diagram.insert(
                    (x,y),
                    *diagram.get(&(x,y)).unwrap_or(&0) + 1
                );

            }
        }
    }

    diagram.values().filter(|&&x| x > 1).count() as u32
}

fn part2(cords: &Vec<Vec<u32>>) -> u32 {

    let mut diagram = HashMap::<(u32,u32), u32>::new();

    for line in cords {
        if line[0] == line[2] || line[1] == line[3] {
            for x in from_to(line[0], line[2]) {
                for y in from_to(line[1], line[3]) {
                    diagram.insert(
                        (x,y),
                        *diagram.get(&(x,y)).unwrap_or(&0) + 1
                    );

                }
            }
        } else {
            for (x,y) in  from_to(line[0], line[2])
                .zip(
                from_to(line[1], line[3])
            ) {
                diagram.insert(
                    (x,y),
                    *diagram.get(&(x,y)).unwrap_or(&0) + 1
                );
            }

        }
    }

    diagram.values().filter(|&&x| x > 1).count() as u32
}