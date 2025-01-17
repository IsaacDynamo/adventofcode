use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};
use std::io::prelude::*;
use std::ops::RangeInclusive;
use std::{collections::HashSet, fs::File};

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

type Input = Vec<Vec<(i32, i32)>>;
type Output = i32;

fn parse(input: &str) -> Input {
    let pair = map(
        separated_pair(digit1, tag(","), digit1),
        |(x, y): (&str, &str)| {
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            (x, y)
        },
    );
    let path = separated_list1(tag(" -> "), pair);
    let mut paths = terminated(separated_list1(tag("\n"), path), opt(tag("\n")));

    let result: nom::IResult<_, _> = paths(input);
    let (_, data) = result.unwrap();
    data
}

fn main() {
    let test = parse(&read_file("test.txt"));
    println!("{:?}", test);
    assert!(dbg!(part1(&test)) == 24);

    let input = parse(&read_file("input.txt"));
    println!("part1: {:?}", part1(&input));

    assert!(dbg!(part2(&test)) == 93);
    println!("part2: {:?}", part2(&input));
}

fn world(input: &Input) -> (HashSet<(i32, i32)>, i32) {
    let mut world = HashSet::new();
    let mut depth = 0;

    fn r(a: i32, b: i32) -> RangeInclusive<i32> {
        a.min(b)..=a.max(b)
    }

    for path in input {
        for segment in path.windows(2) {
            for x in r(segment[0].0, segment[1].0) {
                for y in r(segment[0].1, segment[1].1) {
                    world.insert((x, y));
                    depth = depth.max(y);
                }
            }
        }
    }

    (world, depth)
}

fn step(world: &HashSet<(i32, i32)>, point: (i32, i32)) -> (i32, i32) {
    let nexts = [
        (point.0, point.1 + 1),
        (point.0 - 1, point.1 + 1),
        (point.0 + 1, point.1 + 1),
    ];
    nexts
        .into_iter()
        .filter(|next| !world.contains(next))
        .next()
        .unwrap_or(point)
}

fn move_sand(
    world: &HashSet<(i32, i32)>,
    depth: i32,
    point: (i32, i32),
) -> Result<(i32, i32), (i32, i32)> {
    let mut prev = point;
    loop {
        let next = step(world, prev);
        if next.1 == depth {
            return Err(next);
        }
        if next == prev {
            return Ok(next);
        }
        prev = next;
    }
}

fn part1(input: &Input) -> Output {
    let (mut world, depth) = world(input);
    let start = (500, 0);
    let mut i = 0;

    while let Ok(next) = move_sand(&world, depth, start) {
        world.insert(next);
        i += 1;
    }

    i
}

fn part2(input: &Input) -> Output {
    let (mut world, depth) = world(input);
    let start = (500, 0);
    let mut i = 0;

    loop {
        let next = match move_sand(&world, depth + 1, start) {
            Ok(x) | Err(x) => x,
        };
        world.insert(next);
        i += 1;

        if next == start {
            break;
        }
    }

    i
}
