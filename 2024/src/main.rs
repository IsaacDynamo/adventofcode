use colored::*;
use eyre::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdout;

mod day1;

pub fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

macro_rules! star {
    ($a:expr , $b:expr) => {
        if $a == $b {
            print!("{}", "*".bright_yellow());
        } else {
            print!("_");
        }
        _ = stdout().flush();
    };
}

fn main() -> Result<()> {
    {
        use day1 as day;
        let input = day::parse(&read_file("input/day1/input.txt")?)?;
        star!(day::part1(&input), 2066446);
        star!(day::part2(&input), 24931009);
    }

    println!();

    // Print scale
    for i in 1..=50 {
        if i % 10 == 0 {
            print!("^")
        } else {
            print!(" ")
        }
    }
    println!();

    Ok(())
}
