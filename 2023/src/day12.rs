use eyre::Result;
use std::collections::HashMap;

type Input = Vec<(Vec<char>, Vec<i64>)>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(input
        .lines()
        .map(|line| {
            let (syms, nums) = line.split_once(' ').unwrap();
            let syms = syms.chars().collect();
            let nums = nums.split(',').map(|s| s.parse().unwrap()).collect();
            (syms, nums)
        })
        .collect())
}

pub fn part1(input: &Input) -> Output {
    input
        .iter()
        .map(|(pattern, groups)| solve(pattern, groups))
        .sum()
}

pub fn part2(input: &Input) -> Output {
    let input = input
        .iter()
        .map(|(syms, nums)| {
            let syms = syms
                .iter()
                .chain(['?'].iter())
                .cycle()
                .take(syms.len() * 5 + 4)
                .copied()
                .collect();
            let nums = nums.iter().cycle().take(nums.len() * 5).copied().collect();
            (syms, nums)
        })
        .collect();
    part1(&input)
}

fn solve_inner<'a>(
    cache: &mut HashMap<(&'a [char], &'a [i64]), i64>,
    pattern: &'a [char],
    groups: &'a [i64],
) -> Output {
    // Test for no group
    if groups.is_empty() {
        if pattern.iter().all(|x| *x != '#') {
            return 1;
        } else {
            return 0;
        }
    }

    // Test if there is enough space in the pattern to match the groups.
    // Quick check for early exit
    if pattern.len() < groups.iter().map(|&x| x as usize).sum::<usize>() + groups.len() - 1 {
        return 0;
    }

    // Test for one group
    let g1 = groups[0] as usize;
    if groups.len() == 1 {
        return (0..=pattern.len() - g1)
            .filter(|&i| {
                pattern
                    .get(..i)
                    .unwrap_or_default()
                    .iter()
                    .all(|c| *c != '#')
                    && pattern[i..][..g1].iter().all(|c| *c != '.')
                    && pattern[i..]
                        .get(g1..)
                        .unwrap_or_default()
                        .iter()
                        .all(|c| *c != '#')
            })
            .count() as i64;
    }

    // Use second group as a pivot
    // Slide through the pattern considering every location as the pivot point.
    // Multiply permutations of both halves if pivot is valid
    let g2 = groups[1] as usize;
    let mut sum = 0;
    for i in 0..pattern.len() - g1 - g2 {
        // TODO improve the slide, with a better early exit
        // something like all '#' should be in a g1 window
        if g1 < pattern[..i + g1].iter().filter(|c| **c == '#').count() {
            break;
        }

        // Optional terminator after g2
        let terminator = pattern.get(i + g1 + 1 + g2).copied();
        if pattern[i + g1] != '#'
            && terminator.unwrap_or('.') != '#'
            && pattern[i + g1 + 1..][..g2].iter().all(|c| *c != '.')
        {
            // TODO Combine permutation calc with check for early exit
            let permutations = solve_inner(cache, &pattern[..i + g1], &groups[..=0]);

            if permutations != 0 {
                let pattern_rem = pattern.get(i + g1 + 1 + g2 + 1..).unwrap_or_default();
                sum += permutations * solve_cached(cache, pattern_rem, &groups[2..]);
            }
        }
    }

    sum
}

// The cache allows to reuse calculation
// It deduplicates sub-calculations in the the recursive calculation tree.
fn solve_cached<'a>(
    cache: &mut HashMap<(&'a [char], &'a [i64]), i64>,
    pattern: &'a [char],
    groups: &'a [i64],
) -> Output {
    if let Some(hit) = cache.get(&(pattern, groups)) {
        return *hit;
    }

    let ret = solve_inner(cache, pattern, groups);
    cache.insert((pattern, groups), ret);
    ret
}

fn solve(pattern: &[char], groups: &[i64]) -> Output {
    let mut cache = HashMap::new();
    solve_cached(&mut cache, pattern, groups)
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("day12/test.txt")?)?;
    println!("{:?}", test);

    assert_eq!(part1(&test), 21);
    let input = parse(&read_file("day12/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&test), 525152);
    println!("part2: {}", part2(&input));

    Ok(())
}
