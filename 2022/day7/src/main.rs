use eyre::Result;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = Vec<Cmd>;
type Output = usize;

#[derive(Debug, PartialEq)]
enum Entry {
    File(usize, String),
    Dir(String),
}

#[derive(Debug, PartialEq)]
enum Cmd {
    Cd(String),
    Ls(Vec<Entry>),
}

fn parse(input: &str) -> Result<Input> {
    let mut result = Vec::new();
    for line in input.lines() {
        if line.starts_with("$ cd ") {
            result.push(Cmd::Cd(line.strip_prefix("$ cd ").unwrap().to_string()));
        } else if line.starts_with("$ ls") {
            result.push(Cmd::Ls(Vec::new()));
        } else if line.starts_with("dir ") {
            if let Some(Cmd::Ls(x)) = result.last_mut() {
                x.push(Entry::Dir(line.strip_prefix("dir ").unwrap().to_string()));
            } else {
                panic!();
            }
        } else {
            if let Some(Cmd::Ls(x)) = result.last_mut() {
                let mut parts = line.split_whitespace();
                x.push(Entry::File(parts.next().unwrap().parse().unwrap(), parts.next().unwrap().to_string()));
            } else {
                panic!();
            }
        }
    }
    Ok(result)
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("input.txt")?)?;

    assert!(dbg!(part1(&test)) == 95437);
    println!("part1: {:?}", part1(&input));

    assert!(dbg!(part2(&test)) == 24933642);
    println!("part2: {:?}", part2(&input));

    Ok(())
}

fn part1(input: &Input) -> Output {
    assert!(input.first() == Some(&Cmd::Cd("/".to_string())));
    let mut sum = 0;
    let mut dir_size = Vec::new();
    for cmd in input {
        match cmd {
            Cmd::Cd(x) if x == ".." => {
                let x = dir_size.pop().unwrap();
                if x < 100000 {
                    sum += x;
                }
                *dir_size.last_mut().unwrap() += x;
            },
            Cmd::Cd(_) => { dir_size.push(0); },
            Cmd::Ls(entries) => {
                for entry in entries {
                    match entry {
                        Entry::Dir(_) => (),
                        Entry::File(size, _) => {
                            *dir_size.last_mut().unwrap() += size;
                        },
                    }
                }
            }
        }
    }
    sum
}

fn part2(input: &Input) -> Output {
    assert!(input.first() == Some(&Cmd::Cd("/".to_string())));

    let mut dirs = Vec::new();

    let mut dir_size = Vec::new();
    for cmd in input {
        match cmd {
            Cmd::Cd(x) if x == ".." => {
                let x = dir_size.pop().unwrap();
                dirs.push(x);
                *dir_size.last_mut().unwrap() += x;
            },
            Cmd::Cd(_) => { dir_size.push(0); },
            Cmd::Ls(entries) => {
                for entry in entries {
                    match entry {
                        Entry::Dir(_) => (),
                        Entry::File(size, _) => {
                            *dir_size.last_mut().unwrap() += size;
                        },
                    }
                }
            }
        }
    }

    for _ in 1..dir_size.len() {
        let x = dir_size.pop().unwrap();
        dirs.push(x);
        *dir_size.last_mut().unwrap() += x;
    }

    let root: usize = dir_size.pop().unwrap();
    let needed = root - 40_000_000;

    dirs.iter()
        .copied()
        .filter(|x| *x >= needed)
        .min().unwrap()
}
