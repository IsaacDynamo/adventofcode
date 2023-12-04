use eyre::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = Vec<(i64, Vec<i64>, Vec<i64>)>;
type Output = i64;

fn parse(input: &str) -> Result<Input> {
    Ok(input
        .lines()
        .map(|line| {
            let (game, nums) = line.split_once(':').unwrap();
            let (winning, own) = nums.split_once('|').unwrap();
            let g = game
                .split_whitespace()
                .last()
                .map(|n| n.parse().unwrap())
                .unwrap();
            let w = winning
                .trim()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let o = own
                .trim()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (g, w, o)
        })
        .collect())
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("input.txt")?)?;

    assert!(dbg!(part1(&test)) == 13);
    println!("part1: {}", part1(&input));

    assert!(dbg!(part2(&test)) == 30);
    println!("part2: {}", part2(&input));

    Ok(())
}

fn part1(input: &Input) -> Output {
    input
        .iter()
        .map(|(_, w, o)| {
            let w: HashSet<i64> = HashSet::from_iter(w.iter().copied());
            let o = HashSet::from_iter(o.iter().copied());
            let c = w.intersection(&o).count();
            if c > 0 {
                2_i64.pow(c as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &Input) -> Output {
    let cards = vec![1_i64; input.len()];
    input
        .iter()
        .fold(cards, |mut cards, (i, w, o)| {
            let w: HashSet<i64> = HashSet::from_iter(w.iter().copied());
            let o = HashSet::from_iter(o.iter().copied());
            let c = w.intersection(&o).count() as i64;

            let m = cards[(*i - 1) as usize];
            for n in 0..c {
                cards.get_mut((*i + n) as usize).map(|x| *x += m);
            }

            cards
        })
        .iter()
        .sum()
}
