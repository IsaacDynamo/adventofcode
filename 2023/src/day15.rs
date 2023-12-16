use eyre::Result;

type Input = Vec<String>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input
        .trim()
        .split(',')
        .map(|step| step.to_string())
        .collect())
}

fn hash(input: &str) -> i64 {
    let mut result = 0;
    for c in input.chars() {
        result += c as i64;
        result *= 17;
        result %= 256;
    }
    result
}

pub fn part1(input: &Input) -> Output {
    input.iter().map(|step| hash(step)).sum()
}

enum Step<'a> {
    Add(&'a str, i64),
    Remove(&'a str),
}

fn parse_step(step: &str) -> Step {
    if step.contains('-') {
        Step::Remove(step.strip_suffix('-').unwrap())
    } else {
        let (tag, num) = step.split_once('=').unwrap();
        Step::Add(tag, num.parse().unwrap())
    }
}

pub fn part2(input: &Input) -> Output {
    let mut boxes = vec![Vec::<(&str, i64)>::new(); 256];

    for step in input.iter() {
        let step = parse_step(step);
        match step {
            Step::Add(tag, value) => {
                let b = &mut boxes[hash(tag) as usize];
                if let Some(hit) = b.iter_mut().find(|(t, _)| *t == tag) {
                    hit.1 = value;
                } else {
                    b.push((tag, value));
                }
            }
            Step::Remove(tag) => {
                let b = &mut boxes[hash(tag) as usize];
                b.retain(|(t, _)| *t != tag);
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| -> i64 {
            b.iter()
                .enumerate()
                .map(|(j, (_, v))| (i as i64 + 1) * (j as i64 + 1) * *v)
                .sum()
        })
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("day15/test.txt")?)?;
    println!("{:?}", test);

    assert_eq!(part1(&test), 1320);
    let input = parse(&read_file("day15/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&test), 145);
    println!("part2: {}", part2(&input));

    Ok(())
}
