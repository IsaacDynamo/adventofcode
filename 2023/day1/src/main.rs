use eyre::Result;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = Vec<String>;
type Output = i64;

fn parse(input: &str) -> Result<Input> {
    Ok(input.lines().map(|s| s.to_string()).collect())
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("input.txt")?)?;

    assert!(dbg!(part1(&test)) == 142);
    println!("part1: {}", part1(&input));

    let test = parse(&read_file("test2.txt")?)?;

    assert!(dbg!(part2(&test)) == 281);
    println!("part2: {}", part2(&input));

    Ok(())
}

fn part1(input: &Input) -> Output {
    input
        .iter()
        .map(|line| {
            let mut nums = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|x| x as i64);
            let a = nums.next().unwrap();
            let b = nums.last().unwrap_or(a);
            a * 10 + b
        })
        .sum()
}

static MAP: [(i64, &str); 9] = [
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];

fn num_word(s: &str) -> Option<i64> {
    for (n, name) in MAP.iter() {
        if s.starts_with(name) {
            return Some(*n);
        }
    }
    s.chars()
        .next()
        .and_then(|c| c.to_digit(10))
        .map(|c| c as i64)
}

fn part2(input: &Input) -> Output {
    input
        .iter()
        .map(|line| {
            let mut nums = line.char_indices().filter_map(|(i, _)| {
                let slice = line.as_str().split_at(i).1;
                num_word(slice)
            });
            let a = nums.next().unwrap();
            let b = nums.last().unwrap_or(a);
            a * 10 + b
        })
        .sum()
}
