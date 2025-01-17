use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::iter::Peekable;

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

type Input = Vec<(Packet, Packet)>;
type Output = usize;

fn parse(input: &str) -> Input {
    input.lines().collect::<Vec<&str>>().chunks(3).map(|pair| {
        let a = parse_packet(pair[0]);
        let b = parse_packet(pair[1]);
        (a, b)
    }).collect()
}

#[derive(Debug, Clone)]
enum Packet {
    Num(i32),
    List(Vec<Packet>),
}

fn parse_packet(line: &str) -> Packet {
    fn recurse<T: Iterator<Item=char>>(chr: &mut Peekable<T>) -> Packet {
        let mut list = Vec::<Packet>::new();
        while let Some(c) = chr.next() {
            match c {
                '[' => list.push(recurse(chr)),
                ']' => return Packet::List(list),
                ',' => list.push(recurse(chr)),
                '0'..='9' => {
                    let mut s = String::new();
                    s.push(c);

                    while Some(true) == chr.peek().map(|c| c.is_numeric()) {
                        s.push(chr.next().unwrap());
                    }

                    return Packet::Num(s.parse().unwrap())
                }
                _ => panic!(),
            }
        }
        Packet::List(list)
    }

    let mut lines = line.chars().peekable();
    recurse(&mut lines)
}

fn main() {
    let test = parse(&read_file("test.txt"));
    println!("{:?}", test);
    assert!(dbg!(part1(&test)) == 13);

    let input = parse(&read_file("input.txt"));
    println!("part1: {:?}", part1(&input));

    assert!(dbg!(part2(&test)) == 140);
    println!("part2: {:?}", part2(&input));
}

fn less_vec(a: &[Packet], b: &[Packet]) -> Ordering {
    let mut a = a.iter();
    let mut b = b.iter();
    loop {
        match (a.next(), b.next()) {
            (Some(a), Some(b)) => {
                let r = less(a, b);
                if r.is_ne() { return r }
            },
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return Ordering::Equal,
        }
    }
}

fn less(a: &Packet, b: &Packet) -> Ordering {
    match (a, b) {
        (Packet::Num(a), Packet::Num(b)) => a.cmp(b),
        (Packet::List(a), Packet::List(b)) => less_vec(a, b),
        (Packet::Num(a), Packet::List(b)) => less_vec(&[Packet::Num(*a)], b),
        (Packet::List(a), Packet::Num(b)) => less_vec(a, &[Packet::Num(*b)]),
    }
}

fn part1(input: &Input) -> Output {
    let mut sum = 0;
    for (i, (a, b)) in input.iter().enumerate() {
        if less(a,b) == Ordering::Less {
            sum += i + 1;
        }
    }
    sum
}

fn part2(input: &Input) -> Output {
    let mut all = Vec::new();
    for pair in input {
        all.push(pair.0.clone());
        all.push(pair.1.clone());
    }

    let div = &parse("[[2]]\n[[6]]\n")[0];
    all.push(div.0.clone());
    all.push(div.1.clone());

    all.sort_by(less);

    let x = all.iter().position(|e| less(e, &div.0).is_eq()).unwrap() + 1;
    let y = all.iter().position(|e| less(e, &div.1).is_eq()).unwrap() + 1;

    x  * y
}
