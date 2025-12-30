use eyre::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use string_interner::{StringInterner, backend::StringBackend, symbol::SymbolU32};

type Input = (
    StringInterner<StringBackend>,
    HashMap<SymbolU32, HashSet<SymbolU32>>,
);
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    let mut interner = StringInterner::default();

    let data: HashMap<SymbolU32, HashSet<SymbolU32>> = input
        .lines()
        .map(|l| {
            let (head, tail) = l.split_once(':').unwrap();
            let head = interner.get_or_intern(head);
            let tail = tail
                .trim()
                .split(' ')
                .map(|name| interner.get_or_intern(name))
                .collect();
            (head, tail)
        })
        .collect();

    Ok((interner, data))
}

pub fn part1(input: &Input) -> Output {
    let (interner, data) = input;

    let mut inverse = HashMap::<SymbolU32, HashSet<SymbolU32>>::new();
    data.iter().for_each(|(key, value)| {
        value.iter().for_each(|member| {
            inverse.entry(*member).or_default().insert(*key);
        });
    });

    let start = interner.get("you").unwrap();
    let last = interner.get("out").unwrap();

    let mut order = Vec::new();
    let mut done = HashSet::new();
    let mut work = VecDeque::new();

    let empty = HashSet::new();

    order.push(last);
    done.insert(last);
    inverse
        .get(&last)
        .unwrap()
        .iter()
        .for_each(|x| work.push_back(x));

    while let Some(node) = work.pop_front() {
        if done.contains(node) {
            continue;
        }

        let requires = data.get(node).unwrap();

        if done.is_superset(requires) {
            order.push(*node);
            done.insert(*node);
            inverse
                .get(node)
                .unwrap_or(&empty)
                .iter()
                .for_each(|x| work.push_back(x));
        }
    }

    let mut visitors = HashMap::new();
    visitors.insert(start, 1);

    order.iter().rev().for_each(|x| {
        let n = visitors.get(x).copied().unwrap_or(0);
        data.get(x).unwrap_or(&empty).iter().for_each(|y| {
            *visitors.entry(*y).or_insert(0) += n;
        });
    });

    *visitors.get(&last).unwrap()
}

pub fn part2(input: &Input) -> Output {
    let (interner, data) = input;

    let mut inverse = HashMap::<SymbolU32, HashSet<SymbolU32>>::new();
    data.iter().for_each(|(key, value)| {
        value.iter().for_each(|member| {
            inverse.entry(*member).or_default().insert(*key);
        });
    });

    let start = interner.get("svr").unwrap();
    let last = interner.get("out").unwrap();
    let dac = interner.get("dac").unwrap();
    let fft = interner.get("fft").unwrap();

    let mut order = Vec::new();
    let mut done = HashSet::new();
    let mut work = VecDeque::new();

    let empty = HashSet::new();

    order.push(last);
    done.insert(last);
    inverse
        .get(&last)
        .unwrap()
        .iter()
        .for_each(|x| work.push_back(x));

    while let Some(node) = work.pop_front() {
        if done.contains(node) {
            continue;
        }

        let requires = data.get(node).unwrap();

        if done.is_superset(requires) {
            order.push(*node);
            done.insert(*node);
            inverse
                .get(node)
                .unwrap_or(&empty)
                .iter()
                .for_each(|x| work.push_back(x));
        }
    }

    let mut visitors = HashMap::new();
    visitors.insert(start, 1);

    order.iter().rev().for_each(|x| {
        let n = visitors.get(x).copied().unwrap_or(0);

        if *x == fft || *x == dac {
            visitors.clear();
        }

        data.get(x).unwrap_or(&empty).iter().for_each(|y| {
            *visitors.entry(*y).or_insert(0) += n;
        });
    });

    *visitors.get(&last).unwrap()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("input/day11/example.txt")?)?;
    println!("{:?}", test);
    assert_eq!(part1(&test), 5);

    let input = parse(&read_file("input/day11/input.txt")?)?;
    println!("part1: {}", part1(&input));

    let test = parse(&read_file("input/day11/example2.txt")?)?;
    assert_eq!(part2(&test), 2);
    println!("part2: {}", part2(&input));

    Ok(())
}
