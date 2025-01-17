use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::fmt::{Debug};
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

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

fn parse(input: &str) -> Result<Vec<Vec<u8>>, AppErr> {

    fn parse_line(line: &str) -> Vec<u8> {
        line.chars().map(|c| c as u8 - '0' as u8).collect()
    }

    Ok(input.lines()
        .map( |line| parse_line(line))
        .collect())
}

fn main() -> Result<(), AppErr> {

    let test = parse(&read_file("test.txt")?)?;

    assert!(part1( &test ) == 40);

    let input = parse(&read_file("input.txt")? )?;

    println!("{}", part1( &input ));

    assert!( part2( &test ) == 315);
 
    println!("{}", part2(&input));

    Ok(())
}

fn get(input: &Vec<Vec<u8>>, x: i32, y: i32) -> Option<u8> {

    if x < 0 || y < 0 {
        return None;
    }

    input.get(y as usize)?.get(x as usize).copied()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Route {
    point: (i32, i32),
    risk: u32
}

impl Ord for Route {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Input = Vec<Vec<u8>>;

fn part1( input: &Input) -> u32 {

    type Visited = HashSet::<(i32, i32)>;
    type Queue = BinaryHeap::<Route>;

    let mut queue = Queue::new();
    let mut visited = Visited::new();

    visited.insert((0,0));
    queue.push(Route { point: (0,0), risk: 0 });

    let end = ((input.last().unwrap().len() - 1) as i32, (input.len() - 1) as i32);


    fn step(input: &Input, queue: &mut Queue, visited: &mut Visited, route: Route, dx: i32, dy: i32) {

        let x = route.point.0 + dx;
        let y = route.point.1 + dy;

        if let Some(r) = get(input, x, y) {
            if visited.get( &(x, y) ).is_none() {
                visited.insert( (x,y) );
                queue.push(Route{point: (x,y), risk: route.risk + r as u32 });
            }
        }
    }

    while let Some(best) = queue.pop() {

        if best.point == end {
            return best.risk;
        }

        step(input, &mut queue, &mut visited, best,1,0);
        step(input, &mut queue, &mut visited, best,-1, 0);
        step(input, &mut queue, &mut visited, best,0, 1);
        step(input, &mut queue, &mut visited, best,0,-1);
    } 

    panic!("End not found");
}


fn get2(input: &Vec<Vec<u8>>, x: i32, y: i32) -> Option<u8> {

    if x < 0 || y < 0 {
        return None;
    }

    let xlen = input.last().unwrap().len() as i32;
    let ylen = input.len() as i32;

    if x >= xlen*5  || y >= ylen*5 {
        return None;
    }

    let mut risk = input.get((y % ylen) as usize)?.get((x % xlen) as usize).copied()? as i32;

    risk += x / xlen + y / ylen;

    if risk > 9 {
        risk -= 9;
    }

    Some(risk as u8)
}

fn part2( input: &Input) -> u32 {

    type Visited = HashSet::<(i32, i32)>;
    type Queue = BinaryHeap::<Route>;

    let mut queue = Queue::new();
    let mut visited = Visited::new();

    visited.insert((0,0));
    queue.push(Route { point: (0,0), risk: 0 });

    let end = ((input.last().unwrap().len()*5 - 1) as i32, (input.len()*5 - 1) as i32);


    fn step(input: &Input, queue: &mut Queue, visited: &mut Visited, route: Route, dx: i32, dy: i32) {

        let x = route.point.0 + dx;
        let y = route.point.1 + dy;

        if let Some(r) = get2(input, x, y) {
            if visited.get( &(x, y) ).is_none() {
                visited.insert( (x,y) );
                queue.push(Route{point: (x,y), risk: route.risk + r as u32 });
            }
        }
    }

    while let Some(best) = queue.pop() {

        if best.point == end {
            return best.risk;
        }

        step(input, &mut queue, &mut visited, best,1,0);
        step(input, &mut queue, &mut visited, best,-1, 0);
        step(input, &mut queue, &mut visited, best,0, 1);
        step(input, &mut queue, &mut visited, best,0,-1);
    } 

    panic!("End not found");
}