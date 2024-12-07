use colored::*;
use eyre::Result;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdout;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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

    {
        use day2 as day;
        let input = day::parse(&read_file("input/day2/input.txt")?)?;
        star!(day::part1(&input), 559);
        star!(day::part2(&input), 601);
    }

    {
        use day3 as day;
        let input = day::parse(&read_file("input/day3/input.txt")?)?;
        star!(day::part1(&input), 171183089);
        star!(day::part2(&input), 63866497);
    }

    {
        use day4 as day;
        let input = day::parse(&read_file("input/day4/input.txt")?)?;
        star!(day::part1(&input), 2358);
        star!(day::part2(&input), 1737);
    }

    {
        use day5 as day;
        let input = day::parse(&read_file("input/day5/input.txt")?)?;
        star!(day::part1(&input), 6051);
        star!(day::part2(&input), 5093);
    }

    {
        use day6 as day;
        let input = day::parse(&read_file("input/day6/input.txt")?)?;
        star!(day::part1(&input), 5199);
        star!(day::part2(&input), 1915);
    }

    {
        use day7 as day;
        let input = day::parse(&read_file("input/day7/input.txt")?)?;
        star!(day::part1(&input), 1153997401072);
        star!(day::part2(&input), 97902809384118);
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
