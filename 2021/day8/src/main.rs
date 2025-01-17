use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::fmt::{Debug};
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
enum AppErr {
    IoError(std::io::Error),
}

impl From<std::io::Error> for AppErr {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

fn read_file(path: &str) -> Result<String, AppErr> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


type Entry = (Vec<String>, Vec<String>);

fn parse(input: &str) -> Result<Vec<Entry>, AppErr> {

    fn parse_line(line: &str) -> Entry {
        let mut part = line.split(" | ");
        let a = part.next().unwrap().split(" ").map(|segment| segment.to_string()).collect();
        let b = part.next().unwrap().split(" ").map(|segment| segment.to_string()).collect();
        (a, b)
    }

    Ok(input.lines()
        .map( |line| parse_line(line))
        .collect())
}

fn main() -> Result<(), AppErr> {

    let test = parse(&read_file("test.txt")?)?;

    assert!(part1( &test ) == 26);

    let input = parse(&read_file("input.txt")? )?;

    println!("{}", part1( &input ));

    assert!(part2(&test ) == 61229);
 
    println!("{}", part2(&input));

    Ok(())
}

fn part1( input: &Vec<Entry>) -> u32 {

    let mut count = 0;

    for (_a, b) in input {
        for segment in b {
            match segment.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => ()
            }

        }
    }

    count
}


fn part2( input: &Vec<Entry>) -> u32 {

    fn solve(entry: &Entry ) -> u32 {

        fn overlap( clue: &String, set: &HashSet<char> ) -> usize {
            clue.chars().filter(|x| set.contains(x) ).count()
        }

        fn solve_maybe( clue: &String, digits: &[HashSet<char>; 10] ) -> Option<usize> {
            match clue.len() {
                2 => Some(1),
                3 => Some(7),
                4 => Some(4),
                7 => Some(8),
                5 => { // 2 3 5 

                    //    0 1 2 3 4 5 6 7 8 9
                    // 2: 4 1 5 4 2 3 4 2 5 4
                    // 3: 4 2 4 5 3 4 4 3 5 5
                    // 5: 4 1 3 4 3 5 5 2 5 5

                    if overlap(clue, &digits[2]) == 5 || overlap(clue, &digits[4]) == 2 || overlap(clue, &digits[5]) == 3 || overlap(clue, &digits[9]) == 4 {
                        Some(2)
                    } else
                    if overlap(clue, &digits[1]) == 2 || overlap(clue, &digits[2]) == 4 || overlap(clue, &digits[3]) == 5 || overlap(clue, &digits[5]) == 4 || overlap(clue, &digits[7]) == 3 {
                        Some(3)
                    } else
                    if overlap(clue, &digits[2]) == 3 || overlap(clue, &digits[5]) == 5 || overlap(clue, &digits[6]) == 5 {
                        Some(5)
                    } 
                    else {
                        None
                    }
                }
                6 => { // 0 6 9

                    //    0 1 2 3 4 5 6 7 8 9
                    // 0: 6 2 4 4 3 4 5 3 6 5
                    // 6: 5 1 4 4 3 5 6 2 6 5
                    // 9: 5 2 4 5 4 5 5 3 6 6

                    if overlap(clue, &digits[0]) == 6 || overlap(clue, &digits[5]) == 4 {
                        Some(0)
                    } else 
                    if overlap(clue, &digits[1]) == 1 || overlap(clue, &digits[6]) == 6 || overlap(clue, &digits[7]) == 2 {
                        Some(6)
                    } else 
                    if overlap(clue, &digits[3]) == 5 || overlap(clue, &digits[4]) == 4 || overlap(clue, &digits[9]) == 6  {
                        Some(9)
                    } else {
                        None
                    }
                }
                _ => panic!()
            }
        }

        let mut digits: [HashSet<char>; 10] = Default::default();

        let mut clues = VecDeque::<String>::new();
        clues.extend(entry.0.iter().cloned());
        //clues.extend(entry.1.iter().cloned());

        loop {
            let clue = clues.pop_front();

            if clue.is_none() {
                break;
            }

            let clue = clue.unwrap();

            let x = solve_maybe(&clue, &digits);

            match x {
                Some(i) => 
                    if digits[i].len() == 0 {
                        for c in clue.chars() {
                            digits[i].insert(c);
                        }
                    }
                None => clues.push_back(clue)
            }
        }

        entry.1.iter().fold(0, |a, b| solve_maybe(b, &digits).unwrap() + 10*a ) as u32
    }

    input.iter().map(|entry| solve(entry) ).sum()
}