use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

type Input = Vec<(String, usize, Vec<String>)>;
type Output = usize;

fn parse(input: &str) -> Input {
    input.lines().map(|line| {
        let mut ident_flow_leads = line.split(&['=',';']);
        let ident = ident_flow_leads.next().unwrap().split_whitespace().take(2).last().unwrap().to_string();
        let flow: usize = ident_flow_leads.next().unwrap().parse().unwrap();
        let leads = ident_flow_leads.next().unwrap().split(',').map(|c| {
            c.split_whitespace().last().unwrap().to_string()
        }).collect();

        (ident, flow, leads)
    }).collect()
}

fn main() {
    let test = parse(&read_file("test.txt"));
    println!("{:?}", test);
    assert!(dbg!(part1(&test)) == 1651);

    let input = parse(&read_file("input.txt"));
    println!("part1: {:?}", part1(&input));

    assert!(dbg!(part2(&test)) == 140);
    println!("part2: {:?}", part2(&input));
}

#[derive(Debug)]
struct State {
    score: usize,
    valves: Rc<HashSet<usize>>,
}

fn part1(input: &Input) -> Output {

    let N = input.len();
    let from_name: HashMap<String, usize> = input.iter().enumerate().map(|(i, (ident, _ , _))| (ident.clone(), i)).collect();
    let to_name: Vec<&String> = input.iter().map(|(n,_,_)| n).collect();
    let input: Vec<(usize, Vec<usize>)> = input.iter().map(|(_, flow, leads)| -> (usize, Vec<usize>) {
        let l = leads.iter().map(|n| -> usize { from_name.get(n).copied().unwrap()}).collect();
        (*flow, l)
    }).collect();

    let aa = from_name.get("AA").copied().unwrap();
    let mut scores: Vec<Vec<State>> = (0..N).map(|_| Vec::new()).collect();

    scores[aa].push(State {
        score: 0,
        valves: Rc::new(HashSet::new()),
    });


    for step in 0..30 {

        let total: usize = scores.iter().map(|v| v.len()).sum();
        println!("------------------- {step} {total}");
        for (room, states) in scores.iter().enumerate() {
            let name = to_name[room];
            for state in states {

                let set = state.valves.iter().map(|&r| to_name[r].as_str()).collect::<Vec<&str>>().join(", ");
                println!("{name} {0} [{set}]", state.score)
            }
        }

        let mut new_states: Vec<Vec<State>> = (0..N).map(|_| Vec::new()).collect();

        for (room, states) in scores.iter().enumerate() {
            for state in states {

                let release: usize = state.valves.iter().map(|&room| input[room].0).sum();

                for &moves in input[room].1.iter() {
                    new_states[moves].push( State {
                        score: state.score + release,
                        valves: state.valves.clone(),
                    });
                }

                if !state.valves.contains(&room) && input[room].0 > 0 {
                    let mut v = (*state.valves).clone();
                    v.insert(room);

                    new_states[room].push( State {
                        score: state.score + release,
                        valves: Rc::new(v),
                    });
                }

                // new_states[room].push( State {
                //     score: state.score + release,
                //     valves: state.valves.clone(),
                // });
            }
        }

        let total: usize = new_states.iter().map(|v| v.len()).sum();
        println!("{total}");

        for (room, states) in new_states.into_iter().enumerate() {

            let mut keep_list: Vec<(bool, State)> = Vec::new();

            'skip: for new_state in states {

                for (keep, state) in keep_list.iter_mut() {

                    if *keep == false {
                        continue;
                    }

                    if new_state.score > state.score && new_state.valves.is_superset(&state.valves) {
                        *keep = false;
                        continue;
                    }

                    if new_state.score <= state.score && new_state.valves.is_subset(&state.valves) {
                        continue 'skip;
                    }
                }
                keep_list.push((true, new_state));
            }


            scores[room] = keep_list.into_iter().filter_map(|(keep, e)| keep.then_some(e)).collect();

        }
    }

    scores.iter().map( |v| v.iter().map(|s| {
        s.score
    }).max().unwrap_or(0)).max().unwrap()

}

fn part2(input: &Input) -> Output {
0
}
