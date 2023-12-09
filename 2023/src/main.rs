use eyre::Result;
use std::fs::File;
use std::io::prelude::*;
use colored::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
    };
}

fn main() -> Result<()> {
    let input = day1::parse(&read_file("day1/input.txt")?)?;
    star!(day1::part1(&input), 54877);
    star!(day1::part2(&input), 54100);

    let input = day2::parse(&read_file("day2/input.txt")?)?;
    star!(day2::part1(&input), 2176);
    star!(day2::part2(&input), 63700);

    let input = day3::parse(&read_file("day3/input.txt")?)?;
    star!(day3::part1(&input), 556057);
    star!(day3::part2(&input), 82824352);

    let input = day4::parse(&read_file("day4/input.txt")?)?;
    star!(day4::part1(&input), 24848);
    star!(day4::part2(&input), 7258152);

    let input = day5::parse(&read_file("day5/input.txt")?)?;
    star!(day5::part1(&input), 199602917);
    star!(day5::part2(&input), 2254686);

    let input = day6::parse(&read_file("day6/input.txt")?)?;
    star!(day6::part1(&input), 503424);
    star!(day6::part2(&input), 32607562);

    let input = day7::parse(&read_file("day7/input.txt")?)?;
    star!(day7::part1(&input), 252295678);
    star!(day7::part2(&input), 250577259);

    let input = day8::parse(&read_file("day8/input.txt")?)?;
    star!(day8::part1(&input), 19951);
    star!(day8::part2(&input), 16342438708751);

    let input = day9::parse(&read_file("day9/input.txt")?)?;
    star!(day9::part1(&input), 1934898178);
    star!(day9::part2(&input), 1129);

    println!();

    Ok(())
}
