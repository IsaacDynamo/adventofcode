use colored::*;
use eyre::Result;
use std::fs::File;
use std::io::prelude::*;

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;

mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod grid;

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
        star!(
            2,
            day::part1(&input),
            52316131093,
            day::part2(&input),
            69564213293
        );
    }

    {
        use day3 as day;
        let input = day::parse(&read_file("input/day3/input.txt")?)?;
        star!(
            3,
            day::part1(&input),
            17087,
            day::part2(&input),
            169019504359949
        );
    }

    {
        use day4 as day;
        let input = day::parse(&read_file("input/day4/input.txt")?)?;
        star!(4, day::part1(&input), 1505, day::part2(&input), 9182);
    }

    {
        use day5 as day;
        let input = day::parse(&read_file("input/day5/input.txt")?)?;
        star!(
            5,
            day::part1(&input),
            640,
            day::part2(&input),
            365804144481581
        );
    }

    {
        use day6 as day;
        let input = day::parse(&read_file("input/day6/input.txt")?)?;
        star!(
            6,
            day::part1(&input),
            6725216329103,
            day::part2(&input),
            10600728112865
        );
    }

    {
        use day7 as day;
        let input = day::parse(&read_file("input/day7/input.txt")?)?;
        star!(
            7,
            day::part1(&input),
            1662,
            day::part2(&input),
            40941112789504
        );
    }

    {
        use day8 as day;
        let input = day::parse(&read_file("input/day8/input.txt")?)?;
        star!(
            8,
            day::part1(&input, 1000),
            79560,
            day::part2(&input),
            31182420
        );
    }

    {
        use day9 as day;
        let input = day::parse(&read_file("input/day9/input.txt")?)?;
        star!(
            9,
            day::part1(&input),
            4777816465,
            day::part2(&input),
            1410501884
        );
    }

    {
        use day10 as day;
        let input = day::parse(&read_file("input/day10/input.txt")?)?;
        star!(10, day::part1(&input), 538, day::part2(&input), 20298);
    }

    {
        use day11 as day;
        let input = day::parse(&read_file("input/day11/input.txt")?)?;
        star!(
            11,
            day::part1(&input),
            428,
            day::part2(&input),
            331468292364745
        );
    }

    {
        use day12 as day;
        let input = day::parse(&read_file("input/day12/input.txt")?)?;
        star!(12, day::part1(&input), 448, day::part2(&input), 0);
    }

    Ok(())
}
