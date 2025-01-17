use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::result::Result;
use std::fmt::{Debug};
use std::cmp::{min,max};

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

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32)
}

type Point = (i32, i32);

#[derive(Debug)]
struct Input {
    pub points: HashSet<Point>,
    pub folds: Vec<Fold>
}

fn parse(input: &str) -> Result<Input, AppErr> {

    let mut lines = input.lines();

    let mut points = HashSet::new();
    while let Some(line) = lines.next() {
        let cords: Vec<_> = line.split(",").collect();
        if cords.len() != 2 {
            break;
        }
        points.insert( (cords[0].parse()?, cords[1].parse()?) );
    }

    let mut folds = Vec::new();
    while let Some(line) = lines.next() {

        let instruction: Vec<_> = line.split("=").collect();

        let num = instruction[1].parse()?;

        let res = match instruction[0] {
            "fold along x" => Fold::X(num),
            "fold along y" => Fold::Y(num),
            _ => panic!()
        };

        folds.push(res);
    }

    Ok(Input{points, folds})
}

fn main() -> Result<(), AppErr> {

    let test1 = parse(&read_file("test.txt")?)?;
    assert!(part1( &test1 ) == 17);

    let input = parse(&read_file("input.txt")? )?;
    println!("{}", part1( &input ));

    part2(&input);

    Ok(())
}

fn fold(points: HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    points.into_iter().map( |(x,y)| 
        match fold {
            &Fold::X(q) if x > q => (2*q-x, y),
            &Fold::Y(q) if y > q => (x, 2*q-y),
            _ => (x, y) 
        }
        ).collect()
}

fn part1(input: &Input) -> u32 {

    let points = fold(input.points.clone(), &input.folds[0]);

    points.len() as u32
}

fn part2(input: &Input) {

    let points = input.folds.iter().fold( input.points.clone(), |set, f| fold(set, f) );

    let mut x_min = 1000;
    let mut x_max = -1000;
    let mut y_min = 1000;
    let mut y_max = -1000;

    for &(x,y) in &points {
        x_min = min(x_min, x);
        x_max = max(x_max, x);

        y_min = min(y_min, y);
        y_max = max(y_max, y);
    }

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if points.get(&(x,y)).is_some() {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}