use eyre::Result;
use num::integer::lcm;
use std::collections::HashMap;

type Input = (Vec<char>, Vec<(String, String, String)>);
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    let mut lines = input.lines();

    let dir = lines.next().unwrap().chars().collect();
    _ = lines.next();

    let nodes = lines
        .map(|line| {
            let (node, children) = line.split_once('=').unwrap();
            let pattern = [' ', '(', ')'];
            let (left, right) = children
                .trim_matches(pattern.as_slice())
                .split_once(',')
                .unwrap();
            let node = node.trim().to_string();
            let left = left.trim().to_string();
            let right = right.trim().to_string();
            (node, left, right)
        })
        .collect();

    Ok((dir, nodes))
}

pub fn part1(input: &Input) -> Output {
    let map = input
        .1
        .iter()
        .map(|(n, l, r)| (n, (l, r)))
        .collect::<HashMap<_, _>>();
    let mut pos = &"AAA".to_string();
    for (i, dir) in input.0.iter().cycle().enumerate() {
        let pair = map.get(pos).unwrap();

        pos = match *dir {
            'L' => pair.0,
            _ => pair.1,
        };

        if pos == "ZZZ" {
            return i as i64 + 1;
        }
    }

    panic!("")
}

pub fn part2(input: &Input) -> Output {
    let map: HashMap<&String, (&String, &String)> =
        input.1.iter().map(|(n, l, r)| (n, (l, r))).collect();
    let ghosts: Vec<&String> = input
        .1
        .iter()
        .filter_map(|(n, _, _)| n.ends_with('A').then_some(n))
        .collect();

    let cycles = ghosts
        .iter()
        .map(|&ghost| {
            let mut pos = ghost;
            let mut path = HashMap::new();
            let mut cycle = 0;
            for (i, (c, dir)) in input.0.iter().enumerate().cycle().enumerate() {
                let i = i as i64;

                if let Some(&j) = path.get(&(c, pos)) {
                    cycle = i - j;
                    break;
                }

                _ = path.insert((c, pos), i);

                let pair = map.get(pos).unwrap();
                pos = match *dir {
                    'L' => pair.0,
                    _ => pair.1,
                };
            }

            let mut offsets = path
                .iter()
                .filter_map(|((_, pos), offset)| pos.ends_with('Z').then_some(offset));

            // test3.txt contains a cycle with multiple valid end-nodes. The real input only has one end-node per cycle.
            // So for now filter out the correct offset in the test3.txt case.
            let offset = offsets.find(|offset| cycle == **offset).unwrap();

            (cycle, *offset)
        })
        .collect::<Vec<_>>();

    // In the input file all ghost have the same cycle and offset size. Use this to simplify the search.
    assert!(cycles.iter().all(|(cycle, offset)| cycle == offset));

    cycles.iter().fold(1, |p, (cycle, _)| lcm(p, *cycle))
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test1 = parse(&read_file("day8/test1.txt")?)?;
    println!("{:?}", test1);
    assert!(part1(&test1) == 2);

    let test2 = parse(&read_file("day8/test2.txt")?)?;
    assert!(part1(&test2) == 6);

    let input = parse(&read_file("day8/input.txt")?)?;
    println!("part1: {}", part1(&input));

    let test3 = parse(&read_file("day8/test3.txt")?)?;
    assert!(part2(&test3) == 6);
    println!("part2: {}", part2(&input));

    Ok(())
}
