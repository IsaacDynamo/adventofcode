use colored::*;
use eyre::Result;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
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

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T: Copy> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Grid { data }
    }

    pub fn size(&self) -> (i64, i64) {
        (
            self.data[0].len().try_into().unwrap(),
            self.data.len().try_into().unwrap(),
        )
    }

    pub fn get(&self, x: i64, y: i64) -> Option<T> {
        if x < 0 || y < 0 {
            None
        } else {
            let x: usize = x.try_into().unwrap();
            let y: usize = y.try_into().unwrap();
            self.data.get(y).and_then(|v: &Vec<T>| v.get(x)).copied()
        }
    }

    pub fn get_ref(&self, x: i64, y: i64) -> Option<&T> {
        if x < 0 || y < 0 {
            None
        } else {
            let x: usize = x.try_into().unwrap();
            let y: usize = y.try_into().unwrap();
            self.data.get(y).and_then(|v: &Vec<T>| v.get(x))
        }
    }

    pub fn get_mut(&mut self, x: i64, y: i64) -> Option<&mut T> {
        if x < 0 || y < 0 {
            None
        } else {
            let x: usize = x.try_into().unwrap();
            let y: usize = y.try_into().unwrap();
            self.data.get_mut(y).and_then(|v: &mut Vec<T>| v.get_mut(x))
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (i64, i64, T)> + '_ {
        self.data.iter().enumerate().flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, c)| (x as i64, y as i64, *c))
        })
    }
}

pub static DIR: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

macro_rules! star {
    ($day:expr, $a:expr, $b:expr, $c:expr , $d:expr) => {
        let start = std::time::Instant::now();
        let part1_result = $a == $b;
        let part1_duration = start.elapsed();

        let start = std::time::Instant::now();
        let part2_result = $c == $d;
        let part2_duration = start.elapsed();

        fn token(result: bool) -> ColoredString {
            if result {
                "*".bright_yellow()
            } else {
                "_".into()
            }
        }

        println!(
            "{:>2} {:>12?} {} {:>12?} {}",
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
        star!(1, day::part1(&input), 2066446, day::part2(&input), 24931009);
    }

    {
        use day2 as day;
        let input = day::parse(&read_file("input/day2/input.txt")?)?;
        star!(2, day::part1(&input), 559, day::part2(&input), 601);
    }

    {
        use day3 as day;
        let input = day::parse(&read_file("input/day3/input.txt")?)?;
        star!(
            3,
            day::part1(&input),
            171183089,
            day::part2(&input),
            63866497
        );
    }

    {
        use day4 as day;
        let input = day::parse(&read_file("input/day4/input.txt")?)?;
        star!(4, day::part1(&input), 2358, day::part2(&input), 1737);
    }

    {
        use day5 as day;
        let input = day::parse(&read_file("input/day5/input.txt")?)?;
        star!(5, day::part1(&input), 6051, day::part2(&input), 5093);
    }

    {
        use day6 as day;
        let input = day::parse(&read_file("input/day6/input.txt")?)?;
        star!(6, day::part1(&input), 5199, day::part2(&input), 1915);
    }

    {
        use day7 as day;
        let input = day::parse(&read_file("input/day7/input.txt")?)?;
        star!(
            7,
            day::part1(&input),
            1153997401072,
            day::part2(&input),
            97902809384118
        );
    }

    {
        use day8 as day;
        let input = day::parse(&read_file("input/day8/input.txt")?)?;
        star!(8, day::part1(&input), 423, day::part2(&input), 1287);
    }

    {
        use day9 as day;
        let input = day::parse(&read_file("input/day9/input.txt")?)?;
        star!(
            9,
            day::part1(&input),
            6225730762521,
            day::part2(&input),
            6250605700557
        );
    }

    {
        use day10 as day;
        let input = day::parse(&read_file("input/day10/input.txt")?)?;
        star!(10, day::part1(&input), 514, day::part2(&input), 1162);
    }

    {
        use day11 as day;
        let input = day::parse(&read_file("input/day11/input.txt")?)?;
        star!(
            11,
            day::part1(&input),
            199946,
            day::part2(&input),
            237994815702032
        );
    }

    {
        use day12 as day;
        let input = day::parse(&read_file("input/day12/input.txt")?)?;
        star!(
            12,
            day::part1(&input),
            1483212,
            day::part2(&input),
            0
        );
    }

    {
        use day13 as day;
        let input = day::parse(&read_file("input/day13/input.txt")?)?;
        star!(
            13,
            day::part1(&input),
            29436,
            day::part2(&input),
            103729094227877
        );
    }

    Ok(())
}
