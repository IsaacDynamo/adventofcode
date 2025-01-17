use eyre::Result;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = Vec<Vec<char>>;
type Output = Vec<usize>;

fn parse(input: &str) -> Result<Input> {
    Ok(input.lines().map(|line| line.chars().collect()).collect())
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("input.txt")?)?;

    assert!(dbg!(part1(&test)) == vec![7, 5, 6, 10, 11]);
    println!("part1: {:?}", part1(&input));

    assert!(dbg!(part2(&test)) == vec![19, 23, 23, 29, 26]);
    println!("part2: {:?}", part2(&input));

    Ok(())
}

fn part1(input: &Input) -> Output {
    input.iter().map(|line| {
        for (i, marker) in line.windows(4).enumerate() {
            if marker[0] != marker[1] && marker[0] != marker[2] && marker[0] != marker[3] && marker[1] != marker[2] && marker[1] != marker[3] && marker[2] != marker[3] {
                return i + 4
            }
        }
        panic!()
    }).collect()
}

fn part2(input: &Input) -> Output {
    const N: usize = 14;
    input.iter().map(|line| {
        'search: for (i, marker) in line.windows(N).enumerate() {

            for i in 0..N {
                for j in i+1..N {
                    if marker[i] == marker[j] {
                        continue 'search
                    }
                }
            }

            return i + N
        }
        panic!()
    }).collect()
}
