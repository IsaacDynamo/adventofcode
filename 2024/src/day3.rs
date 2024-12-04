use eyre::Result;
use regex::Regex;

type Input = Vec<String>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input.lines().map(|x| x.to_owned()).collect())
}

pub fn part1(input: &Input) -> Output {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    input
        .iter()
        .map(|line| {
            re.captures_iter(line)
                .map(|m| {
                    let a: i64 = m.get(1).unwrap().as_str().parse().unwrap();
                    let b: i64 = m.get(2).unwrap().as_str().parse().unwrap();
                    a * b
                })
                .sum::<i64>()
        })
        .sum()
}

pub fn part2(input: &Input) -> Output {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    input
        .iter()
        .flat_map(|line| re.captures_iter(line))
        .fold((true, 0), |(enabled, sum), m| {
            match m.get(0).unwrap().as_str() {
                "do()" => (true, sum),
                "don't()" => (false, sum),
                _ => {
                    let r = if enabled {
                        let a: i64 = m.get(1).unwrap().as_str().parse().unwrap();
                        let b: i64 = m.get(2).unwrap().as_str().parse().unwrap();
                        a * b
                    } else {
                        0
                    };
                    (enabled, sum + r)
                }
            }
        })
        .1
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day3/example.txt")?)?;
    let example2 = parse(&read_file("input/day3/example2.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day3/input.txt")?)?;
    println!("input size {}", input.len());

    assert_eq!(part1(&example), 161);
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&example2), 48);
    println!("part2: {}", part2(&input));

    Ok(())
}
