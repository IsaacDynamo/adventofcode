use colored::*;
use eyre::Result;
use std::fs::File;
use std::io::prelude::*;

mod day1;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
mod day2;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;
// mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;

pub fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

macro_rules! star {
    ($day:expr, $a:expr, $b:expr, $c:expr , $d:expr) => {
        let start = std::time::Instant::now();
        let part1_result = $a == $b;
        let part1_duration = start.elapsed().as_micros();

        let start = std::time::Instant::now();
        let part2_result = $c == $d;
        let part2_duration = start.elapsed().as_micros();

        fn token(result: bool) -> ColoredString {
            if result {
                "(*)".bright_yellow()
            } else {
                " . ".into()
            }
        }

        println!(
            "{:>2} {:>12}µs {} {:>12}µs {}",
            $day,
            part1_duration,
            token(part1_result),
            part2_duration,
            token(part2_result)
        );
    };
}

fn main() -> Result<()> {
    {
        use day1 as day;
        let input = day::parse(&read_file("input/day1/input.txt")?)?;
        star!(1, day::part1(&input), 969, day::part2(&input), 5887);
    }

    {
        use day2 as day;
        let input = day::parse(&read_file("input/day2/input.txt")?)?;
        star!(2, day::part1(&input), -1, day::part2(&input), -1);
    }

    Ok(())
}
