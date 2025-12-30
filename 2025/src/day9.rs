use eyre::Result;
use itertools::Itertools;
use rangemap::RangeInclusiveSet;
use std::{
    array,
    collections::{BinaryHeap, HashMap, VecDeque},
};

use crate::grid::Grid;

type Input = Vec<[i64; 2]>;
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

pub fn part1(input: &Input) -> Output {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            input.iter().enumerate().skip(i + 1).map(move |(j, y)| {
                assert_ne!(i, j);

                (x[0].abs_diff(y[0]) + 1) * (x[1].abs_diff(y[1]) + 1)
            })
        })
        .max()
        .unwrap() as i64
}

pub fn part2(input: &Input) -> Output {
    let xs = Vec::from_iter(input.iter().map(|p| p[0]).unique().sorted());
    let ys = Vec::from_iter(input.iter().map(|p| p[1]).unique().sorted());

    let xs_inv: &HashMap<i64, i64> = &xs.iter().enumerate().map(|(i, x)| (*x, i as i64)).collect();
    let ys_inv: &HashMap<i64, i64> = &ys.iter().enumerate().map(|(i, x)| (*x, i as i64)).collect();

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Cell {
        Inner,
        Outer,
        Colored,
    }

    let mut g = Grid::with(
        xs.len() as i64 * 2 + 1,
        ys.len() as i64 * 2 + 1,
        Cell::Inner,
    );

    let mut draw = |a: &[i64; 2], b: &[i64; 2]| {
        if a[0] == b[0] {
            let p = xs_inv.get(&a[0]).copied().unwrap() * 2 + 1;

            let min = a[1].min(b[1]);
            let max = a[1].max(b[1]);

            let q_min = ys_inv.get(&min).copied().unwrap() * 2 + 1;
            let q_max = ys_inv.get(&max).copied().unwrap() * 2 + 1;

            for q in q_min..=q_max {
                *g.get_mut(p, q).unwrap() = Cell::Colored
            }
        } else if a[1] == b[1] {
            let min = a[0].min(b[0]);
            let max = a[0].max(b[0]);

            let p_min = xs_inv.get(&min).copied().unwrap() * 2 + 1;
            let p_max = xs_inv.get(&max).copied().unwrap() * 2 + 1;

            let q = ys_inv.get(&a[1]).copied().unwrap() * 2 + 1;

            for p in p_min..=p_max {
                *g.get_mut(p, q).unwrap() = Cell::Colored
            }
        } else {
            unreachable!()
        }
    };

    input.iter().tuple_windows().for_each(|(a, b)| {
        draw(a, b);
    });

    draw(&input[0], input.last().unwrap());

    let mut work = VecDeque::new();
    work.push_back((0, 0));
    while let Some((x, y)) = work.pop_front() {
        if g.get(x, y) == Some(Cell::Inner) {
            *g.get_mut(x, y).unwrap() = Cell::Outer;

            work.push_back((x - 1, y));
            work.push_back((x + 1, y));
            work.push_back((x, y - 1));
            work.push_back((x, y + 1));
        }
    }

    let mut heap = BinaryHeap::from_iter(input.iter().copied().enumerate().flat_map(|(i, a)| {
        input
            .iter()
            .skip(i + 1)
            .copied()
            .map(move |b| ((a[0].abs_diff(b[0]) + 1) * (a[1].abs_diff(b[1]) + 1), a, b))
    }));

    let mut x_range = vec![RangeInclusiveSet::new(); g.size().1 as usize];
    let mut y_range = vec![RangeInclusiveSet::new(); g.size().0 as usize];

    fn contains_range(s: &RangeInclusiveSet<i64>, a: i64, b: i64) -> bool {
        s.overlapping(a..=b)
            .next()
            .map(|x| *x.start() <= a && b <= *x.end())
            .unwrap_or(false)
    }

    g.iter()
        .filter(|(_, _, cell)| matches!(cell, Cell::Colored | Cell::Inner))
        .for_each(|(x, y, _)| {
            x_range[y as usize].insert(x..=x);
            y_range[x as usize].insert(y..=y);
        });

    (0..)
        .filter_map(|_| heap.pop())
        .find_map(|(s, a, b)| {
            let x_min = a[0].min(b[0]);
            let x_max = a[0].max(b[0]);

            let y_min = a[1].min(b[1]);
            let y_max = a[1].max(b[1]);

            let p_min = xs_inv.get(&x_min).copied().unwrap() * 2 + 1;
            let p_max = xs_inv.get(&x_max).copied().unwrap() * 2 + 1;
            let q_min = ys_inv.get(&y_min).copied().unwrap() * 2 + 1;
            let q_max = ys_inv.get(&y_max).copied().unwrap() * 2 + 1;

            (contains_range(&x_range[q_min as usize], p_min, p_max)
                && contains_range(&x_range[q_max as usize], p_min, p_max)
                && contains_range(&y_range[p_min as usize], q_min, q_max)
                && contains_range(&y_range[p_max as usize], q_min, q_max))
            .then_some(s as i64)
        })
        .unwrap()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("input/day9/example.txt")?)?;
    println!("{:?}", test);
    assert_eq!(part1(&test), 50);

    let input = parse(&read_file("input/day9/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&test), 24);
    println!("part2: {}", part2(&input));

    Ok(())
}
