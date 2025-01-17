use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::fmt::{Debug};

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

type Link = (String, String);

fn parse(input: &str) -> Result<Vec<Link>, AppErr> {

    fn parse_line(line: &str) -> Link {
        let link : Vec<&str> = line.split("-").map(|x|x).collect();
        (link[0].to_string(), link[1].to_string())
    }

    Ok(input.lines()
        .map( |line| parse_line(line))
        .collect())
}

fn main() -> Result<(), AppErr> {

    let test1 = parse(&read_file("test1.txt")?)?;
    assert!(part1( &test1 ) == 19);

    let test2 = parse(&read_file("test2.txt")?)?;
    assert!(part1( &test2 ) == 226);

    let input = parse(&read_file("input.txt")? )?;
    println!("{}", part1( &input ));


    assert!(part2( &test1 ) == 103);
    assert!(part2( &test2 ) == 3509);

    println!("{}", part2(&input));

    Ok(())
}

type Graph = HashMap<String, HashSet<String>>;

fn part1( input: &Vec<Link>) -> u32 {
    
    fn add_link(graph: &mut Graph, from: &str, to: &str) {

        if to == "start" || from == "end" {
            return
        }

        graph.entry(from.to_string()).or_default().insert(to.to_string());

    }

    let mut graph = Graph::new();
    for link in input {
        add_link( &mut graph, &link.0, &link.1);
        add_link( &mut graph, &link.1, &link.0);
    }

    fn explore(graph: &Graph, visited: &mut HashSet<String>, cave: &str) -> u32 {

        if cave == "end" {
            return 1;
        }
        
        visited.insert(cave.to_string());

        let mut paths = 0;


        let d = HashSet::new();

        for connected in graph.get(cave).unwrap_or(&d) {
            
            if connected.chars().all(char::is_lowercase) && visited.get(connected).is_some() {
                continue
            }

            paths += explore(graph, visited, connected);
        }

        visited.remove(cave);

        paths
    }

    let mut visited = HashSet::new();
    explore(&graph, &mut visited, "start")
}


fn part2( input: &Vec<Link>) -> u32 {
    
    fn add_link(graph: &mut Graph, from: &str, to: &str) {

        if to == "start" || from == "end" {
            return
        }

        graph.entry(from.to_string()).or_default().insert(to.to_string());

    }

    let mut graph = Graph::new();
    for link in input {
        add_link( &mut graph, &link.0, &link.1);
        add_link( &mut graph, &link.1, &link.0);
    }

    fn explore(graph: &Graph, visited: &mut HashMap<String, u8>, once: bool, cave: &str) -> u32 {

        if cave == "end" {
            return 1;
        }
        
        *visited.entry(cave.to_string()).or_insert(0) += 1;

        let mut paths = 0;


        let d = HashSet::new();

        for connected in graph.get(cave).unwrap_or(&d) {
            
            let mut joker = once;

            if *visited.get(connected).unwrap_or(&0) != 0 && connected.chars().all(char::is_lowercase) {
                if once {
                    continue
                } else {
                    joker = true;
                }
            }

            paths += explore(graph, visited, joker,connected);
        }

        *(visited.get_mut(cave).unwrap()) -= 1;

        paths
    }

    let mut visited = HashMap::new();
    explore(&graph, &mut visited, false,"start")
}