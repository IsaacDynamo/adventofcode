use eyre::Result;
use std::cmp::max;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = Vec<Vec<i32>>;
type Output = usize;

fn parse(input: &str) -> Result<Input> {
    Ok(input.lines().map(|l|l.chars().map(|c| c.to_string().parse().unwrap()).collect()).collect())
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("input.txt")?)?;

    assert!(dbg!(part1(&test)) == 21);
    println!("part1: {:?}", part1(&input));

    assert!(dbg!(part2(&test)) == 8);
    println!("part2: {:?}", part2(&input));

    Ok(())
}

fn part1(input: &Input) -> Output {
    let width = input[0].len();
    let height = input.len();
    let mut field = vec![vec![0usize; width]; height];

    let mut scan = |i: usize, j: usize, length: &mut i32|{
        if input[j][i] > *length {
            *length = input[j][i];
            field[j][i] = 1;
        }
    };

    for i in 0..width {
        let mut length = [-1i32; 2];
        for j in 0..height {
            scan(i, j, &mut length[0]);
            scan(i, height - 1 -j, &mut length[1]);
        }
    }

    for j in 0..height {
        let mut length = [-1i32; 2];
        for i in 0..width {
            scan(i, j, &mut length[0]);
            scan(width - 1- i, j, &mut length[1]);
        }
    }

    field.iter().map(|x| x.iter().sum::<usize>()).sum()
}

fn part2(input: &Input) -> Output {
    let width = input[0].len();
    let height = input.len();

    let score = |i: usize, j: usize| -> usize {
        let mut prod = [0usize; 4];
        let s = input[j][i];

        prod[0] = (1..=j).find_map(|d| (input[j-d][i] >= s).then_some(d)).unwrap_or(j);
        prod[1] = (1..height - j).find_map(|d| (input[j+d][i] >= s).then_some(d)).unwrap_or(height - 1 - j);
        prod[2] = (1..=i).find_map(|d| (input[j][i-d] >= s).then_some(d)).unwrap_or(i);
        prod[3] = (1..width - i).find_map(|d| (input[j][i+d] >= s).then_some(d)).unwrap_or(width - 1 - i);

        prod.iter().product()
    };

    let mut m = 0;
    for j in 1..(height - 1) {
        for i in 1..(width - 1) {
            m = max(m, score(i, j));
        }
    }
    m
}
