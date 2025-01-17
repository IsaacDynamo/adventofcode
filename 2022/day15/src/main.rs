use eyre::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug)]
struct Pos(i64, i64);
type Input = Vec<(Pos, Pos)>;
type Output = usize;

fn parse(input: &str) -> Result<Input> {
    let r = Regex::new(r"Sensor at x=(-?\d*), y=(-?\d*): closest beacon is at x=(-?\d*), y=(-?\d*)$")?;
    let q = input.lines().map(|line| {
        let capture = r.captures(line).unwrap();
        let a = capture.get(1).map(|x| i64::from_str_radix(x.as_str(), 10).unwrap()).unwrap();
        let b = capture.get(2).map(|x| i64::from_str_radix(x.as_str(), 10).unwrap()).unwrap();
        let c = capture.get(3).map(|x| i64::from_str_radix(x.as_str(), 10).unwrap()).unwrap();
        let d = capture.get(4).map(|x| i64::from_str_radix(x.as_str(), 10).unwrap()).unwrap();
        (Pos(a,b), Pos(c, d))
    }).collect();
    Ok(q)
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    //println!("{:?}", test);
    //assert!(dbg!(part1(10, &test)) == 26);

    let input = parse(&read_file("input.txt")?)?;
    // println!("part1: {:?}", part1(2000000, &input));

    assert!(dbg!(part2(20, &test)) == 56000011);
    println!("part2: {:?}", part2(4000000, &input));

    Ok(())
}

fn distance(a: &Pos, b: &Pos) -> i64 {
    (a.0-b.0).abs() + (a.1-b.1).abs()
}

fn part1(y: i64, input: &Input) -> Output {
    let mut covered = HashSet::<i64>::new();


    for (_, beacon) in input.iter() {
        if beacon.1 == y {
            covered.insert(beacon.0);
        }
    }

    let nbeacons = covered.len();

    for (sensor, beacon) in input.iter() {
        let radius = distance(sensor, beacon);
        let ydiff = (sensor.1-y).abs();
        let xspan = radius - ydiff;

        if xspan >= 0 {
            for v in (sensor.0 - xspan)..=(sensor.0 + xspan) {
                covered.insert(v);
            }
        }

        //println!("{:?} {:?} {} {} {}", sensor, beacon, radius, ydiff, xspan);
    }

    covered.len() - nbeacons
}

fn part2(w: i64, input: &Input) -> Output {

    let mut x = 0;
    let mut y = 0;

    while y < w {
        x = 0;
        'next: while x < w {
            for (sensor, beacon) in input.iter() {
                let radius = distance(sensor, beacon);
                let ydiff = (sensor.1 - y).abs();
                let xspan = radius - ydiff;

                // println!("{} {} {:?} {:?} {} {} {}", x, y, sensor, beacon, radius, ydiff, xspan);

                if x >= sensor.0 - xspan && x <= sensor.0 + xspan {
                    x = sensor.0 + xspan + 1;
                    continue 'next;
                }
            }
            return (x * 4000000 + y) as usize;
        }
        y += 1;
    }

    panic!("Not found");
}