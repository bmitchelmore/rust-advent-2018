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

fn main() {
    let mut map = build_dependency_map();
    let mut steps: Vec<char> = Vec::new();
    loop {
        let next_step = next_step(&map);
        match next_step {
            Some(c) => {
                steps.push(c);
                map = complete_step(c, &map)
            }
            None => break
        }
    }
    let string: String = steps.into_iter().collect();
    println!("step sequence: {:?}", string);
}
