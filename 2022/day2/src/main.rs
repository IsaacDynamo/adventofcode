use std::fs::File;
use std::io::prelude::*;
use eyre::Result;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = Vec<(char, char)>;

fn parse(input: &str) -> Result<Input> {
    let mut rounds = Vec::new();
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        rounds.push((chars[0], chars[2]));
    }
    Ok(rounds)
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    let input = parse(&read_file("input.txt")?)?;

    println!("{:?}", test);

    assert!(dbg!(part1(&test)) == 15);
    println!("part1: {}", part1(&input));

    assert!(dbg!(part2(&test)) == 12);
    println!("part2: {}", part2(&input));

    Ok(())
}

#[derive(PartialEq, Clone, Copy)]
enum Sign {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Sign {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!(),
        }
    }
}

fn part1(input: &Input) -> u32 {
    let mut score = 0;
    for round in input.iter().copied() {
        score += score_round((round.0.into(),round.1.into()));
    }
    score
}

fn score_round(round: (Sign, Sign)) -> u32 {
    use Sign::*;

    let a = match round.1 {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };

    let b = match round {
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
        (x, y) if x == y => 3,
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 0,
        _ => panic!(),
    };

    a + b
}

fn part2(input: &Input) -> u32 {
    use Sign::*;

    const LOSE: char = 'X';
    const DRAW: char = 'Y';
    const WIN: char = 'Z';

    let mut score = 0;
    for round in input.iter().copied() {

        let first = round.0.into();

        let play = match (first, round.1) {
            (Rock, LOSE) => Scissors,
            (Paper, LOSE) => Rock,
            (Scissors, LOSE) => Paper,
            (x, DRAW) => x,
            (Rock, WIN) => Paper,
            (Paper, WIN) => Scissors,
            (Scissors, WIN) => Rock,
            _ => panic!(),
        };

        score += score_round((first, play));
    }
    score
}