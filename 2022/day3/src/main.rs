use eyre::Result;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = Vec<Vec<char>>;

fn parse(input: &str) -> Result<Input> {
    let mut rounds = Vec::new();
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        rounds.push(chars);
    }
    Ok(rounds)
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    let input = parse(&read_file("input.txt")?)?;

    println!("{:?}", test);

    assert!(dbg!(part1(&test)) == 157);
    println!("part1: {}", part1(&input));

    assert!(dbg!(part2(&test)) == 70);
    println!("part2: {}", part2(&input));

    Ok(())
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => panic!(),
    }
}

fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|v| {
            let mid = v.len() / 2;
            let a: HashSet<char, RandomState> = HashSet::from_iter(v[..mid].iter().copied());
            let b: HashSet<char, RandomState> = HashSet::from_iter(v[mid..].iter().copied());

            let overlap: Vec<char> = a.intersection(&b).copied().collect();
            assert!(overlap.len() == 1);
            priority(overlap[0])
        })
        .sum()
}

fn part2(input: &Input) -> u32 {
    input
        .chunks(3)
        .map(|v| {
            assert!(v.len() == 3);

            let a: HashSet<char, RandomState> = HashSet::from_iter(v[0].iter().copied());
            let b: HashSet<char, RandomState> = HashSet::from_iter(v[1].iter().copied());
            let c: HashSet<char, RandomState> = HashSet::from_iter(v[2].iter().copied());

            let overlap: HashSet<char, RandomState> = a.intersection(&b).copied().collect();
            let overlap: Vec<char> = overlap.intersection(&c).copied().collect();

            assert!(overlap.len() == 1);
            priority(overlap[0])
        })
        .sum()
}
