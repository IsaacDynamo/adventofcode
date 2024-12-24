use eyre::{OptionExt, Result};
use petgraph::prelude::*;
use std::collections::HashSet;
use string_interner::{symbol::SymbolU32, StringInterner};

type Input = Vec<(String, String)>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let a = parts.next().ok_or_eyre("-")?.to_string();
            let b = parts.next().ok_or_eyre("-")?.to_string();

            Ok((a, b))
        })
        .collect::<Result<_>>()
}

pub fn part1(input: &Input) -> Output {
    let mut g = UnGraphMap::new();
    let mut intern = StringInterner::default();

    for (a, b) in input {
        let a = intern.get_or_intern(a);
        let b = intern.get_or_intern(b);
        g.add_edge(a, b, ());
    }

    let mut cycles = HashSet::new();
    for n1 in g.nodes() {
        let name = intern.resolve(n1).unwrap();
        if name.starts_with('t') {
            for n2 in g.neighbors(n1) {
                for n3 in g.neighbors(n2) {
                    if g.contains_edge(n3, n1) {
                        let mut cycle = vec![n1, n2, n3];
                        cycle.sort();
                        cycles.insert(cycle);
                    }
                }
            }
        }
    }

    cycles.len().try_into().unwrap()
}

fn grow(
    g: &UnGraphMap<SymbolU32, ()>,
    set: &HashSet<SymbolU32>,
    neighbors: &[SymbolU32],
    best: &mut HashSet<SymbolU32>,
) {
    if set.len() + neighbors.len() <= best.len() {
        // Cannot be better, early out
    } else if neighbors.is_empty() {
        // Base case
        if set.len() > best.len() {
            // Better
            *best = set.clone();
        }
    } else {
        // Try skip
        grow(g, &set, &neighbors[1..], best);

        // Try add
        let n = neighbors[0];
        let c = g.neighbors(n).filter(|x| set.contains(x)).count();
        if set.len() == c {
            let mut set = set.clone();
            set.insert(n);
            grow(g, &set, &neighbors[1..], best);
        }
    }
}

pub fn part2(input: &Input) -> String {
    let mut g = UnGraphMap::new();
    let mut intern = StringInterner::default();

    for (a, b) in input {
        let a = intern.get_or_intern(a);
        let b = intern.get_or_intern(b);
        g.add_edge(a, b, ());
    }

    let mut best = HashSet::new();
    for n in g.nodes() {
        let mut set = HashSet::new();
        set.insert(n);
        let neighbors = g.neighbors(n).filter(|y| *y < n).collect::<Vec<_>>();
        grow(&g, &set, &neighbors, &mut best);
    }

    let mut cluster = best
        .iter()
        .map(|x| intern.resolve(*x).unwrap().to_string())
        .collect::<Vec<_>>();

    cluster.sort();

    let password = cluster
        .into_iter()
        .reduce(|a, b| format!("{},{}", a, b))
        .unwrap();

    password
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day23/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day23/input.txt")?)?;
    println!("input size {:?}", input.len());

    assert_eq!(part1(&example), 7);
    println!("part1: {}", part1(&input));
    assert_eq!(part2(&example), "co,de,ka,ta".to_string());
    println!("part2: {}", part2(&input));

    Ok(())
}
