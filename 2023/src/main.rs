use colored::*;
use eyre::Result;
use std::fs::File;
use std::io::prelude::*;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
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

    let input = day10::parse(&read_file("day10/input.txt")?)?;
    star!(day10::part1(&input), 6682);
    star!(day10::part2(&input), 353);

    let input = day11::parse(&read_file("day11/input.txt")?)?;
    star!(day11::part1(&input), 9536038);
    star!(day11::part2(&input), 447744640566);

    let input = day12::parse(&read_file("day12/input.txt")?)?;
    star!(day12::part1(&input), 7173);
    star!(day12::part2(&input), 29826669191291);

    let input = day13::parse(&read_file("day13/input.txt")?)?;
    star!(day13::part1(&input), 34911);
    star!(day13::part2(&input), 33183);

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
