use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::fmt::{Debug, Display};

#[derive(Debug)]
enum AppErr {
    IoError(std::io::Error),
}

impl From<std::io::Error> for AppErr {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

fn read_file(path: &str) -> Result<String, AppErr> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug, Clone)]
enum Num {
    N (u32),
    P (Box<Num>, Box<Num>)
}

impl Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num::N(i) => write!(f, "{}", i),
            Num::P(a, b) => write!(f, "[{},{}]", a, b)
        }
    }
}

type Input = Vec<Num>;

fn parse(input: &str) -> Result<Input, AppErr> {

    fn p(iter: &mut dyn Iterator<Item=char>) -> Num {

        let c = iter.next().unwrap();
        match c {
            '[' => {
                let a = p(iter);
                assert!(iter.next() == Some(',') );
                let b = p(iter);
                assert!(iter.next() == Some(']') );

                Num::P(Box::new(a), Box::new(b))
            },
            '0'..='9' => {
                Num::N(c.to_digit(10).unwrap())
            },
            _ => panic!(),
        }
    }

    Ok(input.lines().map(|line| p(&mut line.chars()) ).collect::<Vec<Num>>())
}

fn main() -> Result<(), AppErr> {

    fn explode_test(input: &str, result: &str) {
        let mut r = parse(input).unwrap()[0].clone();
        reduce_step(&mut r);
        assert_eq!(r.to_string(), result);
    }

    explode_test("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
    explode_test("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
    explode_test("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
    explode_test("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    explode_test("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
  
    fn magnitude_test(input: &str, result: u32)  {
        let r = &parse(input).unwrap()[0];
        let m = magnitude( r );
        assert_eq!(m, result);
    }

    magnitude_test("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384);
    magnitude_test("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
    magnitude_test("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
    magnitude_test("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137);
    magnitude_test("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488);


    let a = parse("[[[[4,3],4],4],[7,[[8,4],9]]]")?.into_iter().next().unwrap();
    let b = parse("[1,1]")?.into_iter().next().unwrap();
    let mut c = addition(a, b);

    assert_eq!(c.to_string(), "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    reduce_step(&mut c);
    assert_eq!(c.to_string(), "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
    reduce_step(&mut c);
    assert_eq!(c.to_string(), "[[[[0,7],4],[15,[0,13]]],[1,1]]");
    reduce_step(&mut c);
    assert_eq!(c.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
    reduce_step(&mut c);
    assert_eq!(c.to_string(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    reduce_step(&mut c);
    assert_eq!(c.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");


    assert_eq!( fold(parse("[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]")?).to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    assert_eq!( fold(parse("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]")?).to_string(), "[[[[5,0],[7,4]],[5,5]],[6,6]]");

    assert_eq!( fold(parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]\n[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]")?).to_string(), "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]");

    let test2 = parse(&read_file("test2.txt")?)?;
    assert_eq!(fold(test2).to_string(), "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");

    let test = parse(&read_file("test.txt")?)?;
    assert_eq!(part1(test.clone()), 4140);

    let input = parse(&read_file("input.txt")?)?;
    println!("{}", part1(input.clone()));

    assert!( part2( test ) == 3993);

    println!("{}", part2(input));

    Ok(())
}

fn magnitude(num: &Num) -> u32 {
    match num {
        Num::N(n) => *n,
        Num::P(a, b) => 3 * magnitude(a) + 2 * magnitude(b)
    }
}

fn addition(a: Num, b: Num) -> Num {
    Num::P(Box::new(a), Box::new(b))
}

#[derive(Eq, PartialEq, Debug)]
enum Reduce {
    Done,
    Changed,
    Explode(Option<u32>, Option<u32>)
}

fn reduce_step_explode(num: &mut Num, depth: u32) -> Reduce {

    match num {
        Num::N(_) => {
            Reduce::Done
        },
        Num::P(a,b) if depth >= 4 => {
            
            let a = match a.as_ref() {
                Num::N(i) => *i,
                _ => panic!()
            };

            let b = match b.as_ref() {
                Num::N(i) => *i,
                _ => panic!()
            };

            *num = Num::N(0);

            Reduce::Explode(Some(a), Some(b))
        },
        Num::P(a,b) => {
            let depth = depth + 1;
            let r = reduce_step_explode(a, depth);
            match r {
                //Reduce::Changed => return Reduce::Changed,
                Reduce::Done => (),
                Reduce::Explode(p, Some(q)) => {

                    fn add(num: &mut Num, s: u32) {
                        match num {
                            Num::N(i)   => *num = Num::N(*i + s),
                            Num::P(a,_) => add(a, s),
                        }
                    }

                    add(b, q);

                    return Reduce::Explode(p, None)
                },
                _ => return r
            }

            let r = reduce_step_explode(b, depth);
            match r {
                //Reduce::Changed => return Reduce::Changed,
                Reduce::Done => (),
                Reduce::Explode(Some(p), q) => {

                    fn add(num: &mut Num, s: u32) {
                        match num {
                            Num::N(i)   => *num = Num::N(*i + s),
                            Num::P(_,b) => add(b, s),
                        }
                    }

                    add(a, p);

                    return Reduce::Explode(None, q)
                },
                _ => return r
            }

            Reduce::Done
        }
    }
}

fn reduce_step_split(num: &mut Num, depth: u32) -> Reduce {

    match num {
        Num::N(n) if *n > 9 => {
            *num = Num::P( Box::new(Num::N(*n/2)), Box::new(Num::N((*n+1)/2)) );
            Reduce::Changed
        },
        Num::N(_) => {
            Reduce::Done
        },
        Num::P(_,_) if depth >= 4 => {
            panic!()
        },
        Num::P(a,b) => {
            let depth = depth + 1;
            let r = reduce_step_split(a, depth);
            match r {
                Reduce::Done => (),
                _ => return r
            }

            let r = reduce_step_split(b, depth);
            match r {
                Reduce::Done => (),
                _ => return r
            }

            Reduce::Done
        }
    }
}

fn reduce_step(num: &mut Num) -> Reduce {
    let e = reduce_step_explode(num, 0);
    if e != Reduce::Done {
        return e;
    }

    reduce_step_split(num, 0)
}

fn reduce(num: &mut Num) {
    loop {
        let r = reduce_step(num);
        if r == Reduce::Done {
            break
        }
    }
}

fn fold(input: Input) -> Num {
    let mut iter = input.into_iter();
    let mut sum = iter.next().unwrap();

    for i in iter {
        sum = addition(sum, i);
        reduce(&mut sum);
    }

    sum
}

fn part1(input: Input) -> u32 {
    let r = fold(input);
    magnitude(&r)
}

fn part2(input: Input) -> u32 {

    let mut r = Vec::new();
    for a in 0..input.len() {
        for b in 0..input.len() {
            if a != b {
                let v = vec![input[a].clone(), input[b].clone()];
                let m = part1(v);
                r.push(m);
            }
        }
    }

    *r.iter().max().unwrap()
}
