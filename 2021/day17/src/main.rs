use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::fmt::{Debug};
use std::collections::{HashMap, HashSet};

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

type Input = (i32, i32, i32, i32);

fn parse(input: &str) -> Result<Input, AppErr> {

    let parts: Vec<&str> = input.split_terminator(&['=', ',', '.', '\n'][..]).collect();

    let x_min = parts[1].parse::<i32>().unwrap();
    let x_max = parts[3].parse::<i32>().unwrap();
    let y_min = parts[5].parse::<i32>().unwrap();
    let y_max = parts[7].parse::<i32>().unwrap();

    Ok((x_min,x_max,y_min,y_max))
}

fn main() -> Result<(), AppErr> {

    let test = parse("target area: x=20..30, y=-10..-5")?;

    assert!(part1( &test ) == 45);

    let input = parse(&read_file("input.txt")? )?;

    println!("{}", part1( &input ));

    assert!( part2( &test ) == 112);
 
    println!("{}", part2(&input));

    

    let extra1 = parse("target area: x=117..7310, y=-9546..-89")?;
    assert!(part2( &extra1 ) == 69665558);

    let extra2 = parse("target area: x=282184..482382, y=-502273..-374688")?;
    assert!(part2( &extra2 ) == 39067364164);

    Ok(())
}

fn part1(input: &Input) -> i32 {

    // When shot in a arc, the probe will pass through y=0 at some point.
    // The max velocity at y=0 should not exceed y_min-1, otherwise we overshoot the target
    // With the max velocity at y=0 the height can be derived.

    assert!(input.2 < 0);

    let yv0 = -input.2 - 1;

    // yv0 + yv0 - 1 + ... + 2 + 1
    yv0 * (yv0 + 1) / 2
}

fn part2(input: &Input) -> i64 {

    let mut yset = HashMap::<i32, HashSet<i32>>::new();

    for initial_yv in input.2..=-input.2 {
        let mut  y = 0;
        let mut yv = initial_yv;

        for step in 0.. {

            if input.2 <= y && y <= input.3 {
                yset.entry( initial_yv ).or_default().insert(step);
            }

            if y < input.2 {
                break;
            }

            y += yv;
            yv -= 1;
        }
    }

    let step_max = yset.values().map(|x| x.iter().max().unwrap() ).max().copied().unwrap();

    let mut xset = HashMap::<i32, HashSet<i32>>::new();

    for initial_xv in 0..=input.1 {
        let mut  x = 0;
        let mut xv = initial_xv;

        for step in 0..=step_max {

            if input.0 <= x && x <= input.1 {
                xset.entry( initial_xv ).or_default().insert(step);
            }

            if x > input.1 {
                break;
            }

            x += xv;
            if xv != 0 {
                xv -= 1;
            }
        }
    }

    let mut sum = 0;
    for (_, ysteps) in yset.iter() {     
        for (_, xsteps) in xset.iter() {
            if !xsteps.is_disjoint(ysteps) {
                sum += 1;
            }
        }
    }

    sum
}