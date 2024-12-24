use eyre::{OptionExt, Report, Result};
use std::collections::HashMap;

type Input = (Vec<(String, bool)>, Vec<(String, String, String, String)>);
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    let mut lines = input.lines();

    let mut inputs = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let mut input = line.split(':');
        let name = input.next().ok_or_eyre("name")?.to_string();
        let value = input
            .next()
            .ok_or_eyre("value")?
            .trim()
            .parse::<u8>()
            .map_err(Report::from)?;
        inputs.push((name, value != 0))
    }

    let mut gates = Vec::new();
    for line in lines {
        let mut parts = line.split_ascii_whitespace();
        let a = parts.next().ok_or_eyre("")?.to_string();
        let op = parts.next().ok_or_eyre("")?.to_string();
        let b = parts.next().ok_or_eyre("")?.to_string();
        assert_eq!(parts.next(), Some("->"));
        let output = parts.next().ok_or_eyre("")?.to_string();

        gates.push((a, op, b, output));
    }

    Ok((inputs, gates))
}

fn lookup(
    values: &mut HashMap<String, bool>,
    outputs: &HashMap<String, (String, String, String)>,
    id: &str,
) -> bool {
    if let Some(x) = values.get(id) {
        *x
    } else if let Some((op, a, b)) = outputs.get(id) {
        let a = lookup(values, outputs, a);
        let b = lookup(values, outputs, b);
        let x = match op.as_str() {
            "AND" => a && b,
            "OR" => a || b,
            "XOR" => a ^ b,
            _ => unreachable!(),
        };
        values.insert(id.to_string(), x);
        x
    } else {
        unreachable!()
    }
}

pub fn part1(input: &Input) -> Output {
    let mut values = HashMap::from_iter(input.0.iter().cloned());
    let outputs = HashMap::from_iter(
        input
            .1
            .iter()
            .cloned()
            .map(|(a, op, b, out)| (out, (op, a, b))),
    );

    let mut zs = input
        .0
        .iter()
        .map(|(s, _)| s)
        .filter(|s| s.starts_with('z'))
        .cloned()
        .collect::<Vec<_>>();
    zs.extend(
        input
            .1
            .iter()
            .map(|(_, _, _, s)| s)
            .filter(|s| s.starts_with('z'))
            .cloned(),
    );

    zs.iter()
        .map(|z| {
            let v: i64 = lookup(&mut values, &outputs, z).into();
            let p = z.strip_prefix('z').unwrap().parse::<i64>().unwrap();
            v << p
        })
        .sum()
}

pub fn part2(_input: &Input) -> String {
    "".to_string()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day24/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day24/input.txt")?)?;
    println!("input size {} {}", input.0.len(), input.1.len());

    assert_eq!(part1(&example), 2024);
    println!("part1: {}", part1(&input));
    assert_eq!(part2(&example), "z00,z01,z02,z05");
    println!("part2: {}", part2(&input));

    Ok(())
}
