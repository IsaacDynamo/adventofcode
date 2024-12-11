use eyre::{Report, Result};
use itertools::Itertools;
use std::collections::{BTreeMap, VecDeque};

type Input = Vec<i64>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_string().parse::<i64>().map_err(Report::from))
        .collect()
}

pub fn part1(input: &Input) -> Output {
    let mut disk = VecDeque::new();
    let mut id = 0;
    let mut file_block = true;

    for digit in input {
        for _ in 0..*digit {
            if file_block {
                disk.push_back(Some(id));
            } else {
                disk.push_back(None);
            }
        }
        if file_block {
            id += 1;
        }
        file_block = !file_block;
    }

    let mut checksum = 0;
    let mut index = 0;
    while let Some(block) = disk.pop_front() {
        if let Some(id) = block {
            checksum += id * index;
            index += 1;
        } else {
            while let Some(block) = disk.pop_back() {
                if let Some(id) = block {
                    checksum += id * index;
                    index += 1;
                    break;
                }
            }
        }
    }

    checksum
}

pub fn part2(input: &Input) -> Output {
    let mut blocks = Vec::new();
    let mut id: i64 = 0;
    let mut file_block = true;
    let mut index = 0;

    for digit in input {
        if file_block {
            blocks.push((index, id, *digit));
            id += 1;
        }
        index += digit;
        file_block = !file_block;
    }

    let mut disk = BTreeMap::from_iter(blocks.iter().map(|(idx, id, n)| (*idx, (*id, *n))));
    for (idx, _, n) in blocks.iter().rev() {
        // Find gap
        let gap_idx = disk.iter().tuple_windows::<(_, _)>().find_map(
            |((start_idx, (_, m)), (stop_idx, _))| {
                let gap_idx = start_idx + m;
                let gap = stop_idx - gap_idx;
                (gap >= *n).then_some(gap_idx)
            },
        );

        if let Some(new_idx) = gap_idx {
            if new_idx < *idx {
                let file = disk.remove(idx).unwrap();
                disk.insert(new_idx, file);
            }
        }
    }

    disk.iter()
        .map(|(idx, (id, n))| (*idx..(*idx + *n)).map(|i| i * id).sum::<i64>())
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day9/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day9/input.txt")?)?;
    println!("input size {:?}", input.len());

    assert_eq!(part1(&example), 1928);
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&example), 2858);
    println!("part2: {}", part2(&input));

    Ok(())
}
