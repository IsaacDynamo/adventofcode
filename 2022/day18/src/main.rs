use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

type Pos = (i32,i32,i32);
type Input = Vec<Pos>;
type Output = usize;

fn parse(input: &str) -> Input {
    input.lines().map(|line| {
        let mut nums = line.split(',').map(|n| n.parse().unwrap());
        (nums.next().unwrap(), nums.next().unwrap(), nums.next().unwrap())
    }).collect()
}

fn main() {
    let test = parse(&read_file("test.txt"));
    println!("{:?}", test);
    assert!(dbg!(part1(&test)) == 64);

    let input = parse(&read_file("input.txt"));
    println!("part1: {:?}", part1(&input));

    assert!(dbg!(part2(&test)) == 58);
    println!("part2: {:?}", part2(&input));
}

fn part1(input: &Input) -> Output {
    let shape: HashSet<(i32, i32, i32)> = HashSet::from_iter(input.iter().copied());

    let exposed = |x,y,z| -> usize {
        if shape.contains(&(x,y,z)) {
            0
        } else {
            1
        }
    };

    input.iter().map(|&(x,y,z)| {
        let mut sum = 0;
        sum += exposed(x + 1,y,z);
        sum += exposed(x - 1,y,z);
        sum += exposed(x,y + 1,z);
        sum += exposed(x,y - 1,z);
        sum += exposed(x,y,z + 1);
        sum += exposed(x,y,z - 1);
        sum
    }).sum()
}

fn part2(input: &Input) -> Output {

    let x_min = input.iter().map(|&(x, _ , _)| x).min().unwrap() - 1;
    let x_max = input.iter().map(|&(x, _ , _)| x).max().unwrap() + 1;
    let y_min = input.iter().map(|&(_, y , _)| y).min().unwrap() - 1;
    let y_max = input.iter().map(|&(_, y , _)| y).max().unwrap() + 1;
    let z_min = input.iter().map(|&(_, _ , z)| z).min().unwrap() - 1;
    let z_max = input.iter().map(|&(_, _ , z)| z).max().unwrap() + 1;

    let shape: HashSet<Pos> = HashSet::from_iter(input.iter().copied());


    let probe = |fill: &mut Vec<Pos>, visited: &mut HashSet<Pos>, x, y, z| -> usize {
        if !(x_min <= x && x <= x_max && y_min <= y && y <= y_max && z_min <= z && z <= z_max) {
            0
        } else if shape.contains(&(x,y,z)) {
            1
        } else {
            if !visited.contains(&(x,y,z)) {
                fill.push((x,y,z));
            }
            0
        }
    };

    let mut visited= HashSet::new();
    let mut fill = Vec::new();
    fill.push((x_min, y_min, z_min));

    let mut sum = 0;
    while let Some(pos) = fill.pop() {

        if visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);

        let (x,y,z) = pos;

        sum += probe(&mut fill, &mut visited, x + 1, y, z);
        sum += probe(&mut fill, &mut visited, x - 1, y, z);
        sum += probe(&mut fill, &mut visited, x, y + 1, z);
        sum += probe(&mut fill, &mut visited, x, y - 1, z);
        sum += probe(&mut fill, &mut visited, x, y, z + 1);
        sum += probe(&mut fill, &mut visited, x, y, z - 1);
    }
    sum
}