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

fn parse(input: &str) -> Result<Vec<Vec<u8>>, AppErr> {

    fn parse_line(line: &str) -> Vec<u8> {
        line.chars().map(|c| c as u8 - '0' as u8).collect()
    }

    Ok(input.lines()
        .map( |line| parse_line(line))
        .collect())
}

fn main() -> Result<(), AppErr> {

    let test = parse(&read_file("test.txt")?)?;

    assert!(part1( test.clone(), 10 ) == 204);
    assert!(part1( test.clone(), 100 ) == 1656);

    let input = parse(&read_file("input.txt")? )?;

    println!("{}", part1( input.clone(), 100 ));

    assert!( part2( test ) == 195);
 
    println!("{}", part2(input));

    Ok(())
}


fn get(input: &Vec<Vec<u8>>, x: i32, y: i32) -> Option<u8> {

    if x < 0 || y < 0 {
        return None;
    }

    input.get(y as usize)?.get(x as usize).copied()
}

fn part1( mut input: Vec<Vec<u8>>, step: u32) -> u32 {
    
    let mut flashes = 0;

    for _ in 0..step {

        let mut activated = Vec::new();
        
        for y in 0..input.len()  {
            for x in 0..input[0].len() {
                input[y][x] += 1;
                if input[y][x] > 9 {
                    activated.push( (x,y) );
                }
            }
        }

        while let Some( (x,y) ) = activated.pop() {
            
            let x = x as i32;
            let y = y as i32;

            let mut give = |x: i32, y: i32| {
                if let Some(val) = get(&input, x, y) {
                    if val > 9 {
                        return;
                    }

                    let x = x as usize;
                    let y = y as usize;

                    input[y][x] += 1;
                    if input[y][x] > 9 {
                        activated.push( (x ,y) );
                    } 
                }
            };

            give(x - 1, y);
            give(x - 1, y - 1);
            give(x,     y - 1);
            give(x + 1, y - 1);
            give(x + 1, y);
            give(x + 1, y + 1);
            give(x     ,y + 1);
            give(x - 1 ,y + 1);
        }

        for y in 0..input.len()  {
            for x in 0..input[0].len() {
                if input[y][x] > 9 {
                    input[y][x] = 0;
                    flashes += 1;
                }
            }
        }
    }

    flashes
}


fn part2( mut input: Vec<Vec<u8>>) -> u32 {

    for step in 1.. {

        let mut activated = Vec::new();
        
        for y in 0..input.len()  {
            for x in 0..input[0].len() {
                input[y][x] += 1;
                if input[y][x] > 9 {
                    activated.push( (x,y) );
                }
            }
        }

        let mut count = 0;

        while let Some( (x,y) ) = activated.pop() {
            
            count += 1;

            let x = x as i32;
            let y = y as i32;

            let mut give = |x: i32, y: i32| {
                if let Some(val) = get(&input, x, y) {
                    if val > 9 {
                        return;
                    }

                    let x = x as usize;
                    let y = y as usize;

                    input[y][x] += 1;
                    if input[y][x] > 9 {
                        activated.push( (x ,y) );
                    } 
                }
            };

            give(x - 1, y);
            give(x - 1, y - 1);
            give(x,     y - 1);
            give(x + 1, y - 1);
            give(x + 1, y);
            give(x + 1, y + 1);
            give(x     ,y + 1);
            give(x - 1 ,y + 1);
        }

        if count == 100 {
            return step
        }

        for y in 0..input.len()  {
            for x in 0..input[0].len() {
                if input[y][x] > 9 {
                    input[y][x] = 0;
                }
            }
        }
    }

    0
}
