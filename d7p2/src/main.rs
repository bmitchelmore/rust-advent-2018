extern crate regex;

use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Hash)]
struct StepRequirement {
    step: char,
    req: char
}

fn get_requirements() -> Vec<StepRequirement> {
    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
    let file = File::open("input").expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().map(|l| {
        let string = l.expect("string");
        if re.is_match(&string) {
            for cap in re.captures_iter(&string) {
                let req = &cap[1].chars().next().unwrap();
                let step = &cap[2].chars().next().unwrap();
                return StepRequirement { step: *step, req: *req };
            }
        }
        panic!("no match for event")
    }).collect()
}

fn build_dependency_map() -> HashMap<char,Vec<char>> {
    let reqs = get_requirements();
    let mut map: HashMap<char,Vec<char>> = HashMap::new();
    for req in &reqs {
        map.entry(req.req).or_insert(Vec::new());
        let entry = map.entry(req.step).or_insert(Vec::new());
        (*entry).push(req.req)
    }
    return map
}

fn take_step(c: char, map: &HashMap<char,Vec<char>>) -> HashMap<char,Vec<char>> {
    let mut replacement: HashMap<char,Vec<char>> = HashMap::new();
    for (step, deps) in map {
        if *step != c {
            let entry = replacement.entry(*step).or_insert(Vec::new());
            for dep in deps {
                entry.push(*dep)
            }
        }
    }
    return replacement
}

fn complete_step(c: char, map: &HashMap<char,Vec<char>>) -> HashMap<char,Vec<char>> {
    let mut replacement: HashMap<char,Vec<char>> = HashMap::new();
    for (step, deps) in map {
        if *step != c {
            let entry = replacement.entry(*step).or_insert(Vec::new());
            for dep in deps {
                if *dep != c {
                    entry.push(*dep)
                }
            }
        }
    }
    return replacement
}

fn next_step(map: &HashMap<char,Vec<char>>) -> Option<char> {
    let map_iter: Vec<(&char,&Vec<char>)> = map.iter().map(|(k,v)| (k,v)).collect();
    let mut available: Vec<&char> = map_iter.iter().filter(|(_k,v)| v.len() == 0).map(|(k,_v)| *k).collect();
    available.sort_by(|a,b| a.cmp(&b));
    let next_step = available.iter().next();
    match next_step {
        Some(c) => return Some(**c),
        None => return None,
    }
}

fn time_taken_for_step(c: char) -> u8 {
    let mut b = [0; 1];
    let _result = c.encode_utf8(&mut b);
    return *(b.iter().next().unwrap()) - 64 + 60
}

fn tick_forward(worker: &RunningStep) -> RunningStep {
    return RunningStep { step: worker.step, time_remaining: worker.time_remaining - 1 }
}

#[derive(Clone)]
struct RunningStep {
    step: char,
    time_remaining: u8
}

impl RunningStep {
    fn complete(&self) -> bool {
        return self.time_remaining == 0;
    }
}

fn main() {
    let workers = 5;
    let mut time = 0;
    let mut running: Vec<Option<RunningStep>> = vec![None; workers];
    let mut map = build_dependency_map();
    let mut steps: Vec<char> = Vec::new();
    loop {
        for i in 0..workers {
            let worker = &running[i];
            match worker {
                Some(worker) => {
                    let updated = tick_forward(worker);
                    if updated.complete() {
                        map = complete_step(worker.step, &map);
                        running[i] = None;
                    } else {
                        running[i] = Some(updated);
                    }
                },
                None => { 

                }
            }
        }
        for i in 0..workers {
            let worker = &running[i];
            match worker {
                Some(_) => {
                    
                },
                None => {
                    let next_step = next_step(&map);
                    match next_step {
                        Some(c) => {
                            let runner = RunningStep { step: c, time_remaining: time_taken_for_step(c) };
                            println!("starting task {} at time {}. will take {}", runner.step, time, runner.time_remaining);
                            running[i] = Some(runner);
                            steps.push(c);
                            map = take_step(c, &map);
                        }
                        None => {

                        }
                    }
                }
            }
        }
        let pending = running.iter().filter(|k| k.is_none()).count();
        if map.len() == 0 && pending == running.len() {
            break
        } 
        time += 1;
    }
    let string: String = steps.into_iter().collect();
    println!("step sequence: {:?}", string);
    println!("time: {}", time);
}
