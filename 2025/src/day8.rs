use eyre::Result;
use itertools::Itertools;
use std::{array, cmp::Reverse, collections::BinaryHeap};

type Input = Vec<[i64; 3]>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input
        .lines()
        .map(|l| {
            let mut iter = l.split(',').map(|n| n.parse::<i64>().unwrap());
            array::from_fn(|_| iter.next().unwrap())
        })
        .collect())
}

fn find(uf: &mut [usize], i: usize) -> usize {
    let j = uf[i];
    if j == i {
        i
    } else {
        let id = find(uf, j);
        uf[i] = id;
        id
    }
}

pub fn part1(input: &Input, n: usize) -> Output {
    let permutations = input.iter().enumerate().flat_map(|(i, x)| {
        input.iter().enumerate().skip(i + 1).map(move |(j, y)| {
            let d = x
                .iter()
                .zip(y.iter())
                .map(|(&a, &b)| (a - b) * (a - b))
                .sum::<i64>();
            assert_ne!(i, j);
            Reverse((d, i, j))
        })
    });

    let mut heap = BinaryHeap::from_iter(permutations);
    let mut uf: Vec<usize> = input.iter().enumerate().map(|(i, _)| i).collect();

    (0..n)
        .filter_map(|_| heap.pop())
        .for_each(|Reverse((_, i, j))| {
            let a = find(&mut uf, i);
            let b = find(&mut uf, j);
            uf[a] = b;
        });

    let clusters = input
        .iter()
        .enumerate()
        .map(|(i, _)| find(&mut uf, i))
        .counts();
    let mut big = BinaryHeap::from_iter(clusters.values().copied());

    (0..3).filter_map(|_| big.pop()).product::<usize>() as i64
}

pub fn part2(input: &Input) -> Output {
    let permutations = input.iter().enumerate().flat_map(|(i, x)| {
        input.iter().enumerate().skip(i + 1).map(move |(j, y)| {
            let d = x
                .iter()
                .zip(y.iter())
                .map(|(&a, &b)| (a - b) * (a - b))
                .sum::<i64>();

            assert_ne!(i, j);
            Reverse((d, i, j))
        })
    });

    let mut heap = BinaryHeap::from_iter(permutations);
    let mut uf: Vec<usize> = input.iter().enumerate().map(|(i, _)| i).collect();
    let mut clusters = input.len();

    (0..)
        .filter_map(|_| heap.pop())
        .find_map(|Reverse((_, i, j))| {
            let a = find(&mut uf, i);
            let b = find(&mut uf, j);
            if a != b {
                uf[a] = b;
                clusters -= 1;
                if clusters == 1 {
                    return Some(input[i][0] * input[j][0]);
                }
            }

            None
        })
        .unwrap()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("input/day8/example.txt")?)?;
    println!("{:?}", test);
    assert_eq!(part1(&test, 10), 40);

    let input = parse(&read_file("input/day8/input.txt")?)?;

    let a = part1(&input, 1000);
    assert_ne!(a, 97580);
    println!("part1: {}", a);

    assert_eq!(part2(&test), 25272);
    println!("part2: {}", part2(&input));

    Ok(())
}
