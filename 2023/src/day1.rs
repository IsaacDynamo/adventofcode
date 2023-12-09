use eyre::Result;

type Input = Vec<String>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input.lines().map(|s| s.to_string()).collect())
}

pub fn part1(input: &Input) -> Output {
    input
        .iter()
        .map(|line| {
            let mut nums = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|x| x as i64);
            let a = nums.next().unwrap();
            let b = nums.last().unwrap_or(a);
            a * 10 + b
        })
        .sum()
}

static MAP: [(i64, &str); 9] = [
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];

fn num_word(s: &str) -> Option<i64> {
    for (n, name) in MAP.iter() {
        if s.starts_with(name) {
            return Some(*n);
        }
    }
    s.chars()
        .next()
        .and_then(|c| c.to_digit(10))
        .map(|c| c as i64)
}

pub fn part2(input: &Input) -> Output {
    input
        .iter()
        .map(|line| {
            let mut nums = line.char_indices().filter_map(|(i, _)| {
                let slice = line.as_str().split_at(i).1;
                num_word(slice)
            });
            let a = nums.next().unwrap();
            let b = nums.last().unwrap_or(a);
            a * 10 + b
        })
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("day1/test.txt")?)?;
    println!("{:?}", test);
    assert!(part1(&test) == 142);

    let input = parse(&read_file("day1/input.txt")?)?;
    println!("part1: {}", part1(&input));

    let test = parse(&read_file("day1/test2.txt")?)?;

    assert!(part2(&test) == 281);
    println!("part2: {}", part2(&input));

    Ok(())
}
