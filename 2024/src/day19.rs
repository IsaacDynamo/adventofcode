use eyre::Result;
use std::collections::HashMap;

type Input = (Vec<String>, Vec<String>);
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    let mut lines = input.lines();

    let patterns = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    _ = lines.next();
    let designs = lines.map(|s| s.to_string()).collect();

    Ok((patterns, designs))
}

pub fn part1(input: &Input) -> Output {
    fn possible(patterns: &[String], design: &str) -> bool {
        design.is_empty()
            || patterns.iter().any(|s| {
                design
                    .strip_prefix(s)
                    .map(|tail| possible(patterns, tail))
                    .unwrap_or(false)
            })
    }

    input
        .1
        .iter()
        .filter(|design| possible(&input.0, design))
        .count()
        .try_into()
        .unwrap()
}

pub fn part2(input: &Input) -> Output {
    fn possible(cache: &mut HashMap<String, i64>, patterns: &[String], design: &str) -> i64 {
        if let Some(r) = cache.get(design) {
            *r
        } else {
            let r = if design.is_empty() {
                1
            } else {
                patterns
                    .iter()
                    .map(|s| {
                        design
                            .strip_prefix(s)
                            .map(|tail| possible(cache, patterns, tail))
                            .unwrap_or(0)
                    })
                    .sum()
            };
            cache.insert(design.to_string(), r);
            r
        }
    }

    let mut cache = HashMap::new();
    input
        .1
        .iter()
        .map(|design| possible(&mut cache, &input.0, design))
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day19/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day19/input.txt")?)?;
    println!("input size {} {}", input.0.len(), input.1.len());

    assert_eq!(part1(&example), 6);
    println!("part1: {}", part1(&input));
    assert_eq!(part2(&example), 16);
    println!("part2: {}", part2(&input));

    Ok(())
}
