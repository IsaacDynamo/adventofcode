use eyre::Result;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = Vec<Vec<u32>>;

fn parse(input: &str) -> Result<Input> {
    let mut pairs = Vec::new();
    for line in input.lines() {
        let pair: Vec<u32> = line.split([',', '-'].as_slice()).map(|n| n.parse().unwrap()).collect();
        pairs.push(pair);
    }
    Ok(pairs)
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    let input = parse(&read_file("input.txt")?)?;

    println!("{:?}", test);

    assert!(dbg!(part1(&test)) == 2);
    println!("part1: {}", part1(&input));

    assert!(dbg!(part2(&test)) == 4);
    println!("part2: {}", part2(&input));

    Ok(())
}

fn part1(input: &Input) -> u32 {
    input.iter().map(|pair| {
        if (pair[0] <= pair[2] && pair[1] >= pair[3]) || (pair[2] <= pair[0] && pair[3] >= pair[1]) { 1 } else { 0 }
    }).sum()
}

fn part2(input: &Input) -> u32 {
    input.iter().map(|pair| {
        if (pair[1] < pair[2]) || (pair[0] > pair[3]) { 0 } else { 1 }
    }).sum()
}
