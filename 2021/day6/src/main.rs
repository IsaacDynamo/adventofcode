
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

    let fish = parse(&read_file("test.txt")?)?;

    assert!(part1( fish.clone(), 18 ) == 26);
    assert!(part1( fish.clone(), 80 ) == 5934);

    
    let input = parse(&read_file("input.txt")? )?;

    println!("{}", part1( input.clone(), 80 ));

    assert!(part2( fish.clone(), 18 ) == 26);
    assert!(part2( fish.clone(), 80 ) == 5934);
    assert!(part2( fish.clone(), 256 ) == 26984457539);
    assert!(part2( input.clone(), 80 ) == 380243);

    println!("{}", part2(input.clone(), 256));

    Ok(())
}




fn part1(mut fish: Vec<u32>, days: u32) -> u32 {

    for _day in 0..days {

        let mut born = Vec::<u32>::new();
        
        for f in &mut fish {

            if *f == 0 {
                born.push(8);
                *f = 6;
            } else {
                *f -= 1;
            }
        }

        fish.append(&mut born);
    }

    fish.len() as u32
}

fn part2(fish: Vec<u32>, days: u32) -> u64 {

    let mut dist = [0u64; 9];

    for f in fish {
        dist[f as usize] += 1;
    }

    for _day in 0..days {
        dist[7] += dist[0];
        dist.rotate_left(1);
    }

    dist.iter().sum()
}