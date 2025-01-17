use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::fmt::Debug;

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

    assert!(part1( &test ) == 15);

    let input = parse(&read_file("input.txt")? )?;

    assert!(part1( &input ) != 1842);

    println!("{}", part1( &input ));

    assert!(part2(&test ) == 1134);
 
    println!("{}", part2(&input));

    Ok(())
}


fn get(input: &Vec<Vec<u8>>, x: i32, y: i32) -> Option<u8> {

    if x < 0 || y < 0 {
        return None;
    }

    input.get(y as usize)?.get(x as usize).copied()
}

fn part1( input: &Vec<Vec<u8>>) -> u32 {

    let mut sum = 0u32;

    dbg!(get(input, -1, -1));
    dbg!(get(input, 1, 1));
    dbg!(get(input, -1, 1));

    dbg!(get(input, 100, 1));

    dbg!(get(input, 1, 100));


    for y in 0..input.len() as i32 {
        for x in 0..input[0].len() as i32 {

            let val = input[y as usize][x as usize];

            if let Some(t) = get(input, x-1, y){
                if !(val < t) {
                    continue
                }
            }

            if let Some(t) = get(input, x+1, y){
                if !(val < t) {
                    continue
                }
            }

            if let Some(t) = get(input, x, y-1){
                if !(val < t) {
                    continue
                }
            }

            if let Some(t) = get(input, x, y+1){
                if !(val < t) {
                    continue
                }
            }

            println!("{},{}: {}", x, y, val);

            sum += val as u32 + 1;
        }
    }

    sum
}


fn part2( input: &Vec<Vec<u8>>) -> u32 {

    let mut visited: Vec<Vec<bool>> = input
        .iter()
        .map(|x| 
            x.iter()
            .map(|_| false)
            .collect()
        ).collect();


    fn fill(input: &Vec<Vec<u8>>, x: usize, y: usize, visited: &mut Vec<Vec<bool>>) -> u32 {
        let mut work = Vec::new();
        let mut sum = 0;

        let x = x as i32;
        let y = y as i32;


        work.push((x,y));

        while let Some((x,y)) = work.pop() {

            if let Some(val) = get(input, x, y) {

                if visited[y as usize][x as usize] == true {
                    continue
                }

                visited[y as usize][x as usize] = true;

                if val == 9 {
                    continue
                }

                sum += 1;


                work.push((x-1, y));
                work.push((x+1, y));
                work.push((x, y-1));
                work.push((x, y+1));
            }
        }

        sum
    }
    
    let mut basins = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if !visited[y][x] {
                basins.push( dbg!( fill(input, x, y, &mut visited) ) );
            }
        }
    }


    basins.sort();
    basins.iter().rev().take(3).fold(1, |a, x| a * x)
}