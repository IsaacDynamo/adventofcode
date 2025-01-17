use std::io::*;
use std::fs::File;

use regex::Regex;

fn main() {



    let mut file = File::open("input.txt").unwrap();

    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    //let text = String::from("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");


    let valid_cnt : i32 = text.lines().map(|x| is_valid(x) ).sum();
    println!("{:?}", valid_cnt);


    let valid_cnt2 : i32 = text.lines().map(|x| is_valid2(x) ).sum();
    println!("{:?}", valid_cnt2);
}

fn is_valid(line : &str ) -> i32 {

    let re = Regex::new(r#"(\d+)-(\d+) ([a-z]): (\w+)"#).unwrap();

    //println!("{}", line);
    let caps = re.captures(line).unwrap();

    let min : usize = caps.get(1).unwrap().as_str().parse().unwrap();
    let max : usize = caps.get(2).unwrap().as_str().parse().unwrap();
    let letter : char = caps.get(3).unwrap().as_str().parse().unwrap();
    let password = caps.get(4).unwrap().as_str();


    let occurence : usize = password.chars().filter(|&x| x == letter ).count();

    (min <= occurence && occurence <= max).into()
} 

fn is_valid2(line : &str ) -> i32 {

    let re = Regex::new(r#"(\d+)-(\d+) ([a-z]): (\w+)"#).unwrap();

    //println!("{}", line);
    let caps = re.captures(line).unwrap();

    let min : usize = caps.get(1).unwrap().as_str().parse().unwrap();
    let max : usize = caps.get(2).unwrap().as_str().parse().unwrap();
    let letter : char = caps.get(3).unwrap().as_str().parse().unwrap();
    let password = caps.get(4).unwrap().as_str();

    ((password.chars().nth(min -1).unwrap() == letter) ^ (password.chars().nth(max -1).unwrap() == letter)).into()
}