use crate::Grid;
use eyre::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::{cmp::Reverse, collections::BinaryHeap};

type Input = Vec<String>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input.lines().map(|line| line.to_string()).collect())
}

pub fn part1(input: &Input) -> Output {
    let arrowpad = Grid::from_str(" ^A\n<v>");
    let numpad = Grid::from_str("789\n456\n123\n 0A");
    let pads = [&arrowpad, &arrowpad, &numpad];

    solve(&pads, input)
}

pub fn part2(input: &Input) -> Output {
    let arrowpad = Grid::from_str(" ^A\n<v>");
    let numpad = Grid::from_str("789\n456\n123\n 0A");
    let mut pads = vec![&arrowpad; 25];
    pads.push(&numpad);

    solve(&pads, input)
}

fn solve(pads: &[&Grid<char>], input: &Input) -> Output {
    input
        .iter()
        .map(|code| {
            let n = code.strip_suffix('A').unwrap().parse::<i64>().unwrap();
            let l: i64 = seq_len(pads, code).try_into().unwrap();
            n * l
        })
        .sum()
}

fn seq_len(pads: &[&Grid<char>], code: &str) -> usize {
    let mut cost = HashMap::new();
    for a in "A<>^v".chars() {
        for b in "A<>^v".chars() {
            cost.insert((a, b), 1);
        }
    }

    for pad in pads {
        let mut next = HashMap::new();

        for (startx, starty, i) in pad.iter().filter(|&(_, _, c)| c != ' ') {
            let mut visited = HashMap::new();
            let mut queue = BinaryHeap::new();
            queue.push(Reverse((0, startx, starty, 'A')));

            while let Some(Reverse((score, x, y, prev))) = queue.pop() {
                if visited.contains_key(&(x, y, prev)) {
                    continue;
                }

                visited.insert((x, y, prev), score + cost[&(prev, 'A')]);

                let mut step = |delta: (i64, i64), key: char| {
                    if let Some(j) = pad.get(x + delta.0, y + delta.1) {
                        if j != ' ' {
                            queue.push(Reverse((
                                score + cost[&(prev, key)],
                                x + delta.0,
                                y + delta.1,
                                key,
                            )));
                        }
                    }
                };

                step((1, 0), '>');
                step((-1, 0), '<');
                step((0, 1), 'v');
                step((0, -1), '^');
            }

            for (&(x, y, _), &cost) in visited.iter() {
                let j = pad.get(x, y).unwrap();
                let c = next.entry((i, j)).or_insert(cost);
                *c = core::cmp::min(*c, cost);
            }
        }

        cost = next;
    }

    ['A']
        .into_iter()
        .chain(code.chars())
        .tuple_windows::<(_, _)>()
        .map(|pair| cost[&pair])
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day21/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day21/input.txt")?)?;
    println!("input size {} ", input.len());

    assert_eq!(part1(&example), 126384);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));

    Ok(())
}
