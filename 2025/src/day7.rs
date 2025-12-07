use crate::grid::Grid;
use eyre::Result;

type Input = Grid<char>;
type Output = i64;

pub fn parse(input: &str) -> Result<Input> {
    Ok(Grid::from_str(input))
}

pub fn part1(input: &Input) -> Output {
    fn set(v: &mut [bool], i: i64) {
        if i >= 0 && i < v.len() as i64 {
            v[i as usize] = true;
        }
    }

    let (w, h) = input.size();
    let s: Vec<bool> = (0..w).map(|x| input.get(x, 0).unwrap() == 'S').collect();
    let ss = vec![false; s.len()];

    (1..h)
        .fold((s, ss, 0), |(mut s, mut ss, mut splits), y| {
            s.iter()
                .enumerate()
                .filter_map(|(i, x)| (*x).then_some(i as i64))
                .for_each(|x| {
                    let c = input.get(x, y).unwrap();
                    match c {
                        '.' => {
                            set(&mut ss, x);
                        }
                        '^' => {
                            splits += 1;
                            set(&mut ss, x - 1);
                            set(&mut ss, x + 1);
                        }
                        _ => unreachable!(),
                    }
                });
            s.iter_mut().for_each(|x| *x = false);
            (ss, s, splits)
        })
        .2
}

pub fn part2(input: &Input) -> Output {
    fn add(v: &mut [i64], i: i64, n: i64) {
        if i >= 0 && i < v.len() as i64 {
            v[i as usize] += n;
        }
    }

    let (w, h) = input.size();
    let s: Vec<i64> = (0..w)
        .map(|x| (input.get(x, 0).unwrap() == 'S') as i64)
        .collect();
    let ss = vec![0; s.len()];

    (1..h)
        .fold((s, ss), |(mut s, mut ss), y| {
            s.iter().enumerate().for_each(|(x, n)| {
                let x = x as i64;
                let c = input.get(x, y).unwrap();
                match c {
                    '.' => {
                        add(&mut ss, x, *n);
                    }
                    '^' => {
                        add(&mut ss, x - 1, *n);
                        add(&mut ss, x + 1, *n);
                    }
                    _ => unreachable!(),
                }
            });

            s.iter_mut().for_each(|x| *x = 0);
            (ss, s)
        })
        .0
        .iter()
        .sum()
}

#[test]
fn test() -> Result<()> {
    use crate::read_file;

    let test = parse(&read_file("input/day7/example.txt")?)?;
    println!("{:?}", test);
    assert_eq!(part1(&test), 21);

    let input = parse(&read_file("input/day7/input.txt")?)?;
    println!("part1: {}", part1(&input));

    assert_eq!(part2(&test), 40);
    println!("part2: {}", part2(&input));

    Ok(())
}
