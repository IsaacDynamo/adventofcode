use std::io::*;
use std::fs::File;
use itertools::*;

fn main() {
    println!("Hello, world!");


    let mut file = File::open("input.txt").unwrap();

    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();


    let numbers : Vec<i32> = text.lines().map(|x| x.parse().unwrap() ).collect();
    
    let result : Vec<i32> = numbers.iter().permutations(2).filter(|x| x.iter().fold(0, |acc, x| acc + **x) == 2020) .map(|y| y.iter().fold(1, |prod, x| prod * **x) ).collect();

    println!("{:?}", result);

    let result : Vec<i32> = numbers.iter().permutations(3).filter(|x| x.iter().fold(0, |acc, x| acc + **x) == 2020) .map(|y| y.iter().fold(1, |prod, x| prod * **x) ).collect();

    println!("{:?}", result);

}
