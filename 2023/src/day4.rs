use eyre::Result;
use std::collections::HashSet;

type Input = Vec<(i64, Vec<i64>, Vec<i64>)>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
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
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let o = own.split_whitespace().map(|n| n.parse().unwrap()).collect();
            (g, w, o)
        })
        .collect())
}

pub fn part1(input: &Input) -> Output {
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

pub fn part2(input: &Input) -> Output {
    let cards = vec![1_i64; input.len()];
    input
        .iter()
        .fold(cards, |mut cards, (i, w, o)| {
            let w: HashSet<i64> = HashSet::from_iter(w.iter().copied());
            let o = HashSet::from_iter(o.iter().copied());
            let c = w.intersection(&o).count() as i64;

            let m = cards[(*i - 1) as usize];
            for n in 0..c {
                if let Some(x) = cards.get_mut((*i + n) as usize) {
                    *x += m;
                }
            }

            cards
        })
        .iter()
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("day4/test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("day4/input.txt")?)?;

    assert!(part1(&test) == 13);
    println!("part1: {}", part1(&input));

    assert!(part2(&test) == 30);
    println!("part2: {}", part2(&input));

    Ok(())
}
