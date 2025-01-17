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


fn parse(input: &str) -> Result<Vec<u32>, AppErr> {
    Ok(input.lines()
        .next().unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap() )
        .collect())
}

fn main() -> Result<(), AppErr> {

    let test = parse(&read_file("test.txt")?)?;

    assert!(part1( test.clone() ) == 37);

    let input = parse(&read_file("input.txt")? )?;

    println!("{}", part1( input.clone() ));

    assert!(fuel2(&test,2) == 206);

    assert!(part2(&test ) == 168);
 
    println!("{}", part2(&input));

    Ok(())
}


fn fuel(crabs: &Vec<u32>, depth: u32) -> u32 {
    crabs.iter().map( |&x| (x as i64 - depth as i64).abs() as u32 ).sum()
}


fn part1(mut crabs: Vec<u32>) -> u32 {

    crabs.sort();

    let median = crabs[crabs.len()/2];

    dbg!(fuel(&crabs, median-1) );
    dbg!(fuel(&crabs, median+1) );

    dbg!(fuel(&crabs, median) )
}


fn cost(change: u32) -> u32 {
    match change {
        0 => 0,
        _ => change + cost(change - 1)
    }
}


fn fuel2(crabs: &Vec<u32>, depth: u32) -> u32 {
    crabs.iter().map( |&x| cost( (x as i64 - depth as i64).abs() as u32) ).sum()
}

fn part2(crabs: &Vec<u32>) -> u32 {


    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    dbg!(min, max);

    let mut best = (max, fuel2(&crabs, max));
    
    for i in min..max {
        let c = fuel2(&crabs, i);

        if c < best.1 {
            best = (i, c);
        }

    }
    
    dbg!(best);

    best.1
}