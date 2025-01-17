use std::fs::File;
use std::io::prelude::*;
use eyre::Result;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse(input: &str) -> Result<Vec<Vec<u32>>> {
    let mut groups = Vec::new();
    groups.push(vec![]);
    for line in input.lines() {
        if line == "" {
            groups.push(vec![]);
        } else {
            groups.last_mut().unwrap().push(line.parse()?);
        }
    }
    Ok(groups)
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    let input = parse(&read_file("input.txt")?)?;

    assert!(dbg!(part1(&test)) == 24000);
    println!("part1: {}", part1(&input));

    assert!(dbg!(part2(&test)) == 45000);
    println!("part2: {}", part2(&input));

    Ok(())
}

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    input.iter().map(|v| v.iter().sum()).max().unwrap()
}

fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let mut calories: Vec<u32> = input.iter().map(|v| v.iter().sum()).collect();
    calories.sort();
    calories.iter().rev().take(3).sum()
}