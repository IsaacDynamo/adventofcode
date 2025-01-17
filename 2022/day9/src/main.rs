use eyre::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

type Input = Vec<(char, i32)>;
type Output = usize;

fn parse(input: &str) -> Result<Input> {
    Ok(input.lines().map(|l| {
        let mut parts = l.split_whitespace();
        let a = parts.next().unwrap().chars().next().unwrap();
        let n = parts.next().unwrap().parse().unwrap();
        (a, n)
    }).collect())
}

fn main() -> Result<()> {
    let test = parse(&read_file("test.txt")?)?;
    println!("{:?}", test);

    let input = parse(&read_file("input.txt")?)?;

    assert!(dbg!(part1(&test)) == 13);
    println!("part1: {:?}", part1(&input));

    assert!(dbg!(part2(&test)) == 1);
    let test2 = parse(&read_file("test2.txt")?)?;
    assert!(dbg!(part2(&test2)) == 36);

    println!("part2: {:?}", part2(&input));

    Ok(())
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

fn part1(input: &Input) -> Output {
    let mut trail = HashSet::<Pos>::new();
    let mut head = Pos::default();
    let mut tail= Pos::default();

    trail.insert(tail);

    for &(action, count) in input {
        for _ in 0..count {
            match action {
                'U' => head.y += 1,
                'D' => head.y -= 1,
                'R' => head.x += 1,
                'L' => head.x -= 1,
                _ => panic!(),
            }

            let dx = head.x - tail.x;
            let dy = head.y - tail.y;

            assert!(dx.abs() + dy.abs() <= 3, "{head:?} {tail:?}");

            if dx == -2 {
                tail.x -= 1;
                tail.y = head.y;
            }

            if dx == 2 {
                tail.x += 1;
                tail.y = head.y;
            }

            if dy == -2 {
                tail.y -= 1;
                tail.x = head.x;
            }

            if dy == 2 {
                tail.y += 1;
                tail.x = head.x;
            }

            trail.insert(tail);
        }
    }

    trail.len()
}


fn pull(head: Pos, tail: &mut Pos) {
    let dx = head.x - tail.x;
    let dy = head.y - tail.y;

    assert!(dx.abs() + dy.abs() <= 4, "{head:?} {tail:?}");

    match (dx, dy) {
        (x, y) if x.abs() == 2 && y.abs() == 2 => {
            tail.x += x/2;
            tail.y += y/2;
        },
        (x, _) if x.abs() == 2 => {
            tail.x += x/2;
            tail.y = head.y;
        },
        (_, y) if y.abs() == 2 => {
            tail.y += y/2;
            tail.x = head.x;
        },
        _ => (),
    }
}

fn part2(input: &Input) -> Output {
    let mut trail = HashSet::<Pos>::new();
    let mut rope = [Pos::default(); 10];

    trail.insert(*rope.last().unwrap());

    for &(action, count) in input {
        for _ in 0..count {
            match action {
                'U' => rope[0].y += 1,
                'D' => rope[0].y -= 1,
                'R' => rope[0].x += 1,
                'L' => rope[0].x -= 1,
                _ => panic!(),
            }

            for i in 1..rope.len() {
                pull(rope[i - 1], &mut rope[i]);
            }

            trail.insert(*rope.last().unwrap());
        }
    }

    trail.len()
}
