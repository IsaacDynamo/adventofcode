use std::collections::{HashMap, HashSet, VecDeque};

use eyre::{Report, Result};

type Input = (Vec<(i64, i64)>, Vec<Vec<i64>>);
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    let mut lines = input.lines();
    let mut pairs = Vec::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let pair = line
            .split('|')
            .map(|n| n.parse::<i64>().map_err(Report::from))
            .collect::<Result<Vec<i64>>>()?;

        pairs.push((pair[0], pair[1]))
    }

    let mut lists = Vec::new();
    for line in lines {
        let list = line
            .split(',')
            .map(|n| n.parse::<i64>().map_err(Report::from))
            .collect::<Result<Vec<i64>>>()?;
        lists.push(list);
    }

    Ok((pairs, lists))
}

type Constraints = HashMap<i64, HashSet<i64>>;

fn make_constraints(input: &Input) -> Constraints {
    let mut constraints: Constraints = HashMap::new();

    for (a, b) in input.0.iter() {
        if let Some(s) = constraints.get_mut(b) {
            s.insert(*a);
        } else {
            constraints.insert(*b, HashSet::from_iter([*a].into_iter()));
        }
    }

    constraints
}

fn is_ordered(list: &[i64], constraints: &Constraints) -> bool {
    list.iter()
        .try_fold(HashSet::<i64>::new(), |mut acc, x| {
            if acc.contains(x) {
                None
            } else {
                if let Some(c) = constraints.get(x) {
                    acc.extend(c.iter());
                }
                Some(acc)
            }
        })
        .is_some()
}

pub fn part1(input: &Input) -> Output {
    let constraints = make_constraints(input);
    input
        .1
        .iter()
        .filter(|list| is_ordered(list, &constraints))
        .map(|x| x[x.len() / 2])
        .sum()
}

pub fn part2(input: &Input) -> Output {
    fn backtrack(
        cs: &HashMap<i64, HashSet<i64>>,
        rem: HashSet<i64>,
        c: HashSet<i64>,
    ) -> Option<VecDeque<i64>> {
        if rem.is_empty() {
            return Some(VecDeque::new());
        }

        if !rem.is_disjoint(&c) {
            return None;
        }

        rem.iter().find_map(|x| {
            let mut rem = rem.clone();
            let mut c = c.clone();
            rem.remove(x);
            if let Some(q) = cs.get(x) {
                c.extend(q.iter());
            }
            backtrack(cs, rem, c).map(|mut v| {
                v.push_front(*x);
                v
            })
        })
    }

    let constraints = make_constraints(input);

    input
        .1
        .iter()
        .filter(|list| !is_ordered(list, &constraints))
        .map(|list| {
            let set = HashSet::from_iter(list.iter().copied());
            assert_eq!(set.len(), list.len());
            let sorted = backtrack(&constraints, set, HashSet::new()).unwrap();
            sorted[sorted.len() / 2]
        })
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day5/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day5/input.txt")?)?;
    println!("input size {} {}", input.0.len(), input.1.len());

    assert_eq!(part1(&example), 143);
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&example), 123);
    println!("part2: {}", part2(&input));

    Ok(())
}

#[test]
fn has_cycles() -> Result<()> {
    use crate::read_file;

    let input = parse(&read_file("input/day5/input.txt")?)?;

    let mut g = petgraph::prelude::DiGraphMap::new();
    for (a, b) in input.0.iter() {
        g.add_edge(a, b, ());
    }
    assert!(!petgraph::algo::is_cyclic_directed(&g));

    Ok(())
}
