use eyre::Result;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::ops::AddAssign;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = Vec<(Vec<char>, i64)>;
type Output = i64;

fn parse(input: &str) -> Result<Input> {
    Ok(input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            (cards.chars().collect(), bid.parse().unwrap())
        })
        .collect())
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("input.txt")?)?;

    assert!(dbg!(part1(&test)) == 6440);
    println!("part1: {}", part1(&input));

    assert!(dbg!(part2(&test)) == 5905);
    println!("part2: {}", part2(&input));

    Ok(())
}

fn part1(input: &Input) -> Output {
    fn map(c: char) -> usize {
        static M: [char; 13] = [
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ];
        M.iter().position(|m| *m == c).unwrap()
    }

    fn score(cards: &[char]) -> usize {
        let bag = cards.iter().fold(HashMap::new(), |mut bag, c| {
            bag.entry(*c).or_insert(0_i64).add_assign(1);
            bag
        });

        let mut counts = bag.iter().map(|(_, i)| *i).collect::<Vec<i64>>();
        counts.sort_by_key(|x| std::cmp::Reverse(*x));

        match *counts.as_slice() {
            [5] => 6,
            [4, 1] => 5,
            [3, 2] => 4,
            [3, 1, 1] => 3,
            [2, 2, 1] => 2,
            [2, 1, 1, 1] => 1,
            [1, 1, 1, 1, 1] => 0,
            _ => panic!("wut"),
        }
    }

    let mut sorted = input.clone();
    sorted.sort_by(|(a, _), (b, _)| {
        let o = score(a).cmp(&score(b));
        if o == Ordering::Equal {
            let a = a.iter().map(|c| map(*c));
            let b = b.iter().map(|c| map(*c));
            b.partial_cmp(a).unwrap()
        } else {
            o
        }
    });

    sorted
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as i64 + 1) * *bid)
        .sum()
}

fn part2(input: &Input) -> Output {
    fn map(c: char) -> usize {
        static M: [char; 13] = [
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
        ];
        M.iter().position(|m| *m == c).unwrap()
    }

    fn score(cards: &[char]) -> usize {
        let mut bag = cards.iter().fold(HashMap::new(), |mut bag, c| {
            bag.entry(*c).or_insert(0_i64).add_assign(1);
            bag
        });

        let jokers = bag.remove(&'J').unwrap_or(0);

        let mut counts = bag.iter().map(|(_, i)| *i).collect::<Vec<i64>>();
        counts.sort_by_key(|x| std::cmp::Reverse(*x));

        if counts.is_empty() {
            counts.push(0);
        }
        counts[0] += jokers;

        match *counts.as_slice() {
            [5] => 6,
            [4, 1] => 5,
            [3, 2] => 4,
            [3, 1, 1] => 3,
            [2, 2, 1] => 2,
            [2, 1, 1, 1] => 1,
            [1, 1, 1, 1, 1] => 0,
            _ => panic!("wut"),
        }
    }

    let mut sorted = input.clone();
    sorted.sort_by(|(a, _), (b, _)| {
        let o = score(a).cmp(&score(b));
        if o == Ordering::Equal {
            let a = a.iter().map(|c| map(*c));
            let b = b.iter().map(|c| map(*c));
            b.partial_cmp(a).unwrap()
        } else {
            o
        }
    });

    sorted
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as i64 + 1) * *bid)
        .sum()
}
