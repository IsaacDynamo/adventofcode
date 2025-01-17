use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::fmt::Debug;
use std::collections::{HashSet, HashMap, VecDeque};

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

type Coordinate = (i32,i32,i32);
type Scanner = Vec<Coordinate>;
type Input = Vec<Scanner>;


fn parse(input: &str) -> Result<Input, AppErr> {
    let mut r = Input::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("---") {
            r.push(vec![]);
            continue;
        }

        let numbers: Vec<i32> = line.split(",").map(|x| x.parse().unwrap() ).collect();

        let cord = (numbers[0],numbers[1],numbers[2]);
        r.last_mut().unwrap().push(cord);

    }

    Ok(r)
}

const EXAMPLE1: &str = 
r#"--- scanner 0 ---
0,2,0
4,1,0
3,3,0

--- scanner 1 ---
-1,-1,0
-5,0,0
-2,1,0"#;

const EXAMPLE2: &str =
r#"--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0

--- scanner 0 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8

--- scanner 0 ---
1,1,-1
2,2,-2
3,3,-3
1,3,-2
-4,-6,5
7,0,8

--- scanner 0 ---
1,1,1
2,2,2
3,3,3
3,1,2
-6,-4,-5
0,7,-8
"#;

const EXAMPLE3: &str = 
r#"--- scanner 0 ---
-618,-824,-621
-537,-823,-458
-447,-329,318
404,-588,-901
544,-627,-890
528,-643,409
-661,-816,-575
390,-675,-793
423,-701,434
-345,-311,381
459,-707,401
-485,-357,347

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
-476,619,847
-460,603,-452
729,430,532
-322,571,750
-355,545,-477
413,935,-424
-391,539,-444
553,889,-390
"#;

fn main() -> Result<(), AppErr> {

    let example1 = parse(EXAMPLE1)?;
    assert_eq!(part12(&example1, 3).0, 3);

    let example2 = parse(EXAMPLE2)?;
    assert_eq!(part12(&example2, 6).0, 6);

    let example3 = parse(EXAMPLE3)?;
    assert_eq!(part12(&example3, 12).0, 12);

    let test = parse(&read_file("test.txt")?)?;
    assert_eq!(part12(&test, 12), (79, 3621));

    let input = parse(&read_file("input.txt")?)?;
    println!("{:?}", part12(&input, 12));

    Ok(())
}

fn add(a: Coordinate, b: Coordinate) -> Coordinate {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn sub(a: Coordinate, b: Coordinate) -> Coordinate {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn dist(a: Coordinate, b: Coordinate) -> u32 {
    (i32::abs(a.0 - b.0) + i32::abs(a.1 - b.1) + i32::abs(a.2 - b.2)) as u32
}

const FACES: [ &dyn Fn (Coordinate) -> Coordinate; 6] = [
    &|(x,y,z)| (x,y,z),
    &|(x,y,z)| (z,y,-x),
    &|(x,y,z)| (-z,y,x),
    &|(x,y,z)| (-y,x,z),
    &|(x,y,z)| (y,-x,z),
    &|(x,y,z)| (-x,-y,z),
];

const ROTATIONS: [ &dyn Fn (Coordinate) -> Coordinate; 4] = [
    &|(x,y,z)| (x,y,z),
    &|(x,y,z)| (x,-z,y),
    &|(x,y,z)| (x,-y,-z),
    &|(x,y,z)| (x,z,-y),
];

fn part12(input: &Input, threshold: u32) -> (u32,u32) {

    let mut scanners = input.iter();
    let mut beacons: HashSet::<Coordinate> = scanners.next().unwrap().iter().copied().collect();
    let mut location_scanners = vec![(0,0,0)];

    let mut scanners: VecDeque<_> = scanners.enumerate().collect();

    while let Some((s,scanner)) = scanners.pop_front() {
        let mut hit = Vec::new();
        for (f, &face) in FACES.iter().enumerate() {
            for (r, &rot) in ROTATIONS.iter().enumerate() {
            
                let mut location = HashMap::<Coordinate, u32>::new();

                for &a in beacons.iter() {
                    for &b in scanner {
                        *location.entry(sub(a,rot(face(b)))).or_insert(0) += 1;
                    }
                }

                let hot = location.iter().filter(|&(_,&i)| i >= threshold).collect::<Vec<_>>();
                assert!(hot.len() <= 1, "There should be at most 1 hot location");

                if let Some(&(&cord, _)) = hot.get(0) {
                    hit.push( (cord, f, r) )
                }
            }
        }

        if hit.len() == 1 {

            let (offset, f, r) = hit[0];
            let rot = ROTATIONS[r];
            let face = FACES[f];

            beacons.extend(scanner.iter().map(|&a| add(offset, rot(face(a)))));
            location_scanners.push(offset);
        } else {
            scanners.push_back((s, scanner));
        }

    }

    let mut max = 0;
    for (i, &a) in location_scanners.iter().enumerate() {
        for (j, &b) in location_scanners.iter().enumerate() {
            if i < j {
                max = u32::max(max, dist(a,b));
            }
        }
    }

    (beacons.len() as u32, max)
}

