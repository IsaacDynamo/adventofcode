use crate::Grid;
use eyre::Result;
use std::collections::HashSet;

type Input = Grid<char>;
type Output = i64;
type Point = (i64, i64);

pub fn parse(input: &str) -> Result<Input> {
    Ok(Grid::new(
        input.lines().map(|x| x.chars().collect()).collect(),
    ))
}

pub fn part1(input: &Input) -> Output {
    let size = input.size();
    let mut nodes = HashSet::new();

    for (i, (x1, y1, c1)) in input.iter().enumerate() {
        for (j, (x2, y2, c2)) in input.iter().enumerate() {
            if i < j && c1 == c2 && c1 != '.' {
                fn insert(nodes: &mut HashSet<Point>, size: Point, point: Point) {
                    if 0 <= point.0 && point.0 < size.0 && 0 <= point.1 && point.1 < size.1 {
                        nodes.insert(point);
                    }
                }

                insert(&mut nodes, size, (2 * x1 - x2, 2 * y1 - y2));
                insert(&mut nodes, size, (2 * x2 - x1, 2 * y2 - y1));
            }
        }
    }

    nodes.len().try_into().unwrap()
}

pub fn part2(input: &Input) -> Output {
    let size = input.size();
    let mut nodes = HashSet::new();

    for (i, (x1, y1, c1)) in input.iter().enumerate() {
        for (j, (x2, y2, c2)) in input.iter().enumerate() {
            if i < j && c1 == c2 && c1 != '.' {
                fn insert(nodes: &mut HashSet<Point>, size: Point, mut point: Point, delta: Point) {
                    while 0 <= point.0 && point.0 < size.0 && 0 <= point.1 && point.1 < size.1 {
                        nodes.insert(point);
                        point = (point.0 + delta.0, point.1 + delta.1);
                    }
                }

                insert(&mut nodes, size, (x1, y1), (x1 - x2, y1 - y2));
                insert(&mut nodes, size, (x2, y2), (x2 - x1, y2 - y1));
            }
        }
    }

    nodes.len().try_into().unwrap()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day8/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day8/input.txt")?)?;
    println!("input size {:?}", input.size());

    assert_eq!(part1(&example), 14);
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&example), 34);
    println!("part2: {}", part2(&input));

    Ok(())
}
