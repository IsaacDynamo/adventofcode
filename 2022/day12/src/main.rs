use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

type Input = Vec<Vec<char>>;
type Output = i32;

fn parse(input: &str) -> Input {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn main() {
    let test = parse(&read_file("test.txt"));
    println!("{:?}", test);
    assert!(dbg!(part1(&test)) == 31);

    let input = parse(&read_file("input.txt"));
    println!("part1: {:?}", part1(&input));

    assert!(dbg!(part2(&test)) == 29);
    println!("part2: {:?}", part2(&input));
}

#[derive(Debug, Eq, PartialEq)]
struct Node (i32, i32, i32);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.2.cmp(&other.2).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get(i: &Input, x: i32, y: i32) -> Option<char> {
    if x < 0 || y < 0 {
        return None;
    }
    i.get(y as usize)?.get(x as usize).copied()
}

fn map(x: char) -> u32 {
    let x = match x {
        'E' => 'z',
        'S' => 'a',
        x => x,
    };
    x as u32
}

fn part1(input: &Input) -> Output {
    let mut start = None;
    for (y, v) in input.iter().enumerate() {
        let y = y as i32;
        for (x, c) in v.iter().enumerate() {
            let x = x as i32;
            if *c == 'S' {
                start = Some((x, y));
            }
        }
    }

    let start = start.unwrap();

    let mut visited = HashMap::new();
    let mut head = BinaryHeap::new();

    head.push(Node (start.0, start.1, 0));

    while let Some(node) = head.pop() {

        if visited.get(&(node.0, node.1)).is_some() {
            continue
        }

        visited.insert((node.0, node.1), node.2);

        let s = get(input, node.0, node.1).unwrap();

        if s == 'E' {
            return node.2;
        }

        let mut visit = |x, y| {
            if visited.get(&(x, y)).is_none() {
                if let Some(o) = get(input, x, y) {
                    if map(o) <= map(s) + 1 {
                        head.push(Node (x, y, node.2 + 1));
                    }
                }
            }
        };

        visit(node.0, node.1 + 1);
        visit(node.0, node.1 - 1);
        visit(node.0 + 1, node.1);
        visit(node.0 - 1, node.1);
    }

    0
}

fn part2(input: &Input) -> Output {
    let mut visited = HashMap::new();
    let mut head = BinaryHeap::new();

    for (y, v) in input.iter().enumerate() {
        let y = y as i32;
        for (x, c) in v.iter().enumerate() {
            let x = x as i32;
            if map(*c) == 'a' as u32 {
                head.push(Node (x, y, 0));
            }
        }
    }

    while let Some(node) = head.pop() {

        if visited.get(&(node.0, node.1)).is_some() {
            continue
        }

        visited.insert((node.0, node.1), node.2);

        let s = get(input, node.0, node.1).unwrap();

        if s == 'E' {
            return node.2;
        }

        let mut visit = |x, y| {
            if visited.get(&(x, y)).is_none() {
                if let Some(o) = get(input, x, y) {
                    if map(o) <= map(s) + 1 {
                        head.push(Node (x, y, node.2 + 1));
                    }
                }
            }
        };

        visit(node.0, node.1 + 1);
        visit(node.0, node.1 - 1);
        visit(node.0 + 1, node.1);
        visit(node.0 - 1, node.1);
    }

    0
}
