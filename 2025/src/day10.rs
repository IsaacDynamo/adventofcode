use eyre::Result;
use good_lp::{Expression, Solution, SolverModel, Variable, default_solver, variable, variables};
use nom::{
    IResult, Parser,
    bytes::complete::take_while,
    character::complete::{char, digit1},
    combinator::all_consuming,
    multi::separated_list0,
    sequence::{delimited, preceded},
};
use std::collections::{HashMap, HashSet, VecDeque, hash_map};

type Item = (String, Vec<Vec<i64>>, Vec<i64>);
type Input = Vec<Item>;
type Output = i64;

fn parse_line(input: &str) -> IResult<&str, Item> {
    let indicators = delimited(
        char('['),
        take_while(|c| c != ']').map(|s: &str| s.to_string()),
        char(']'),
    );

    let buttons = preceded(
        char(' '),
        separated_list0(
            char(' '),
            delimited(
                char('('),
                separated_list0(char(','), digit1.map(|s: &str| s.parse().unwrap())),
                char(')'),
            ),
        ),
    );

    let joltage = preceded(
        char(' '),
        delimited(
            char('{'),
            separated_list0(char(','), digit1.map(|s: &str| s.parse().unwrap())),
            char('}'),
        ),
    );

    all_consuming((indicators, buttons, joltage)).parse(input)
}

pub fn parse(input: &str) -> Result<Input> {
    Ok(input.lines().map(|l| parse_line(l).unwrap().1).collect())
}

pub fn part1(input: &Input) -> Output {
    input
        .iter()
        .map(|(pattern, buttons, _)| {
            let end = pattern
                .chars()
                .enumerate()
                .map(|(i, x)| ((x == '#') as u64) << i)
                .fold(0, |a, b| a | b);
            let start = 0;

            let buttons: Vec<u64> = buttons
                .iter()
                .map(|wires| wires.iter().map(|x| 1u64 << x).fold(0, |a, b| a | b))
                .collect();

            let mut work = VecDeque::new();
            work.push_back((start, 0));

            let mut visited = HashMap::new();
            while let Some((p, n)) = work.pop_front() {
                if let hash_map::Entry::Vacant(e) = visited.entry(p) {
                    e.insert(n);
                    buttons.iter().for_each(|x| {
                        work.push_back((p ^ *x, n + 1));
                    })
                }

                if p == end {
                    break;
                }
            }

            *visited.get(&end).unwrap()
        })
        .sum()
}

pub fn part2(input: &Input) -> Output {
    input
        .iter()
        .map(|(_, buttons, joltage)| {
            let mut variables = variables!();
            let vars: Vec<Variable> = buttons
                .iter()
                .map(|_| variables.add(variable().integer().min(0)))
                .collect();

            let objective = vars.iter().fold(Expression::from(0), |a, b| a + b);

            let buttons: Vec<HashSet<i64>> = buttons
                .iter()
                .map(|c| HashSet::from_iter(c.iter().copied()))
                .collect();

            let constraints = joltage.iter().enumerate().map(|(i, j)| {
                let i = i as i64;
                let sum = buttons
                    .iter()
                    .zip(vars.iter())
                    .filter(|(button, _)| button.contains(&i))
                    .map(|(_, var)| var)
                    .fold(Expression::from(0), |a, b| a + b);

                let c = Expression::from(*j as i32);
                good_lp::constraint::eq(sum, c)
            });

            let solution = variables
                .minimise(&objective)
                .using(default_solver)
                .with_all(constraints)
                .solve()
                .unwrap();

            solution.eval(&objective).round() as i64
        })
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("input/day10/example.txt")?)?;
    println!("{:?}", test);
    assert_eq!(part1(&test), 7);

    let input = parse(&read_file("input/day10/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&test), 33);
    println!("part2: {}", part2(&input));

    Ok(())
}
