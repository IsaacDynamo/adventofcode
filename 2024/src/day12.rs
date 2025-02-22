use crate::Grid;
use eyre::Result;
use std::collections::{HashMap, HashSet};

type Input = Grid<char>;
type Output = i64;
type Point = (i64, i64);

pub fn parse(input: &str) -> Result<Input> {
    Ok(Grid::new(
        input.lines().map(|x| x.chars().collect()).collect(),
    ))
}

#[derive(Debug, Clone, Copy)]
enum Node {
    Link(Point),
    Leaf(Leaf),
}

#[derive(Debug, Clone, Copy)]
struct Leaf {
    area: i64,
    perimeter: i64,
}

fn get(nodes: &Grid<Node>, x: i64, y: i64) -> Option<&Leaf> {
    if let Some(Node::Link((x, y))) = nodes.get_ref(x, y) {
        //print!("+");
        get(nodes, *x, *y)
    } else {
        //println!();
        match nodes.get_ref(x, y) {
            Some(Node::Leaf(leaf)) => Some(leaf),
            Some(Node::Link(_)) => unreachable!(),
            None => None,
        }
    }
}

fn get_mut(nodes: &mut Grid<Node>, x: i64, y: i64) -> Option<&mut Leaf> {
    if let Some(Node::Link((x, y))) = nodes.get(x, y) {
        get_mut(nodes, x, y)
    } else {
        match nodes.get_mut(x, y) {
            Some(Node::Leaf(leaf)) => Some(leaf),
            Some(Node::Link(_)) => unreachable!(),
            None => None,
        }
    }
}

fn get_mut_node(nodes: &mut Grid<Node>, x: i64, y: i64) -> Option<&mut Node> {
    if let Some(Node::Link((x, y))) = nodes.get(x, y) {
        get_mut_node(nodes, x, y)
    } else {
        match nodes.get_mut(x, y) {
            Some(n) => Some(n),
            None => None,
        }
    }
}

fn compress(nodes: &Grid<Node>, a: Point) -> Point {
    if let Some(Node::Link(p)) = nodes.get_ref(a.0, a.1) {
        compress(nodes, *p)
    } else {
        match nodes.get_ref(a.0, a.1) {
            Some(Node::Leaf(_)) => a,
            Some(Node::Link(_)) => unreachable!(),
            None => unreachable!(),
        }
    }
}

fn unify(nodes: &mut Grid<Node>, a: Point, b: Point) {
    let c = get(nodes, a.0, a.1);
    let d = get(nodes, b.0, b.1);

    match (c, d) {
        (Some(c), Some(d)) if !std::ptr::eq(c, d) => {
            let d = *d;
            *(get_mut_node(nodes, b.0, b.1).unwrap()) = Node::Link(compress(nodes, a));
            let n = get_mut(nodes, a.0, a.1).unwrap();
            n.area += d.area;
            n.perimeter += d.perimeter;
            n.perimeter -= 2;
        }
        (Some(c), Some(d)) if std::ptr::eq(c, d) => {
            let n = get_mut(nodes, a.0, a.1).unwrap();
            n.perimeter -= 2;
        }
        _ => (),
    }
}

pub fn part1(input: &Input) -> Output {
    let mut nodes = input.map(|_, _, _| {
        Node::Leaf(Leaf {
            area: 1,
            perimeter: 4,
        })
    });

    for (x, y, c) in input.iter() {
        if input.get(x - 1, y) == Some(c) {
            unify(&mut nodes, (x - 1, y), (x, y));
        }

        if input.get(x, y - 1) == Some(c) {
            unify(&mut nodes, (x, y - 1), (x, y));
        }
    }

    nodes
        .iter()
        .filter_map(|(_, _, n)| match n {
            Node::Link(_) => None,
            Node::Leaf(Leaf { area, perimeter }) => Some(area * perimeter),
        })
        .sum()
}

fn probe(
    input: &Input,
    region: &mut HashSet<Point>,
    edges: &mut HashSet<(Point, Point)>,
    pos: Point,
    dir: Point,
    c: char,
) {
    let next = (pos.0 + dir.0, pos.1 + dir.1);
    if region.contains(&next) {
        return;
    }

    if Some(c) == input.get(next.0, next.1) {
        region.insert(next);
        probe4(input, region, edges, next, c);
    } else {
        edges.insert((pos, dir));
    }
}

fn probe4(
    input: &Input,
    region: &mut HashSet<Point>,
    edges: &mut HashSet<(Point, Point)>,
    pos: Point,
    c: char,
) {
    probe(input, region, edges, pos, (1, 0), c);
    probe(input, region, edges, pos, (-1, 0), c);
    probe(input, region, edges, pos, (0, 1), c);
    probe(input, region, edges, pos, (0, -1), c);
}

fn set_pop(set: &mut HashSet<(Point, Point)>) -> Option<(Point, Point)> {
    let ret = set.iter().next().copied();

    if let Some(e) = ret {
        set.remove(&e);
    }

    ret
}

fn map_pop(map: &mut HashMap<Point, char>) -> Option<(Point, char)> {
    map.keys()
        .next()
        .copied()
        .and_then(|k| map.remove(&k).map(|v| (k, v)))
}

fn add(a: Point, b: Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
}

fn clock(dir: Point) -> Point {
    (-dir.1, dir.0)
}

fn counter(dir: Point) -> Point {
    (dir.1, -dir.0)
}

pub fn part2(input: &Input) -> Output {
    let mut remaining = input
        .iter()
        .map(|(x, y, c)| ((x, y), c))
        .collect::<HashMap<(i64, i64), char>>();

    let mut result = 0;
    while let Some((pos, c)) = map_pop(&mut remaining) {
        let mut region = HashSet::new();
        let mut edges = HashSet::new();

        region.insert(pos);
        probe4(input, &mut region, &mut edges, pos, c);

        for pos in &region {
            remaining.remove(pos);
        }

        let mut sides = 0;
        while let Some((pos, dir)) = set_pop(&mut edges) {
            sides += 1;

            let mut next = add(pos, counter(dir));
            while edges.remove(&(next, dir)) {
                next = add(next, counter(dir));
            }

            let mut next = add(pos, clock(dir));
            while edges.remove(&(next, dir)) {
                next = add(next, clock(dir));
            }
        }

        let area: i64 = region.len().try_into().unwrap();
        result += area * sides;
    }

    result
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let example = parse(&read_file("input/day12/example.txt")?)?;
    println!("example: {:?}", example);
    let input = parse(&read_file("input/day12/input.txt")?)?;
    println!("input size {:?}", input.size());

    assert_eq!(part1(&example), 1930);
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&example), 1206);
    println!("part2: {}", part2(&input));

    Ok(())
}
