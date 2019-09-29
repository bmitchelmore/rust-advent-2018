extern crate regex;

use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;

#[derive(Debug)]
struct Pot {
    has_plant: bool
}

#[derive(Debug)]
struct State {
    pots: Vec<Pot>
}

#[derive(Debug)]
struct Rule {
    state: [bool; 5],
    result: bool
}

#[derive(Debug)]
struct Rules {
    rules: Vec<Rule>
}

impl State {
    fn new(s: &str) -> State {
        let cap = s.len() * 2 - 1;
        let mut vec = Vec::with_capacity(cap);
        for _i in 1..s.len() {
            vec.push(Pot { has_plant: false });
        }
        for c in s.chars() {
            let has_plant = c == '#';
            vec.push(Pot { has_plant: has_plant });
        }
        State { pots: vec }
    }
    fn iterate_times(&mut self, rules: &Rules, n: u32) {
        for _i in 0..n {
            self.iterate(rules);
        }
    }
    fn iterate(&mut self, rules: &Rules) {
        for rule in &rules.rules {
            println!("{:?}", rule);
        }
    }
}

fn state_from_string(s: &str) -> bool {
    return s == "#"
}

impl Rule {
    fn new(s: &str) -> Option<Rule> {
        let tcre = Regex::new(r"([#.])([#.])([#.])([#.])([#.]) => ([#.])").unwrap();
        if tcre.is_match(&s) {
            for cap in tcre.captures_iter(&s) {
                let state1_str = &cap[1];
                let state2_str = &cap[2];
                let state3_str = &cap[3];
                let state4_str = &cap[4];
                let state5_str = &cap[5];
                let result_str = &cap[6];
                let state1_bool = state_from_string(state1_str);
                let state2_bool = state_from_string(state2_str);
                let state3_bool = state_from_string(state3_str);
                let state4_bool = state_from_string(state4_str);
                let state5_bool = state_from_string(state5_str);
                let result_bool = state_from_string(result_str);
                let rule = Rule { state: [state1_bool, state2_bool, state3_bool, state4_bool, state5_bool], result: result_bool };
                return Some(rule);
            }
        }
        None
    }
}

fn get_state_and_rules() -> (State, Rules) {
    let isre = Regex::new(r"initial state: ([#.]+)").unwrap();
    let file = File::open("input").expect("File not found");
    let buf = BufReader::new(file);
    let mut lines = buf.lines();
    let first = lines.next().unwrap().expect("string");
    if isre.is_match(&first) {
        for cap in isre.captures_iter(&first) {
            let val = &cap[1];
            let state = State::new(val);
            let rules = lines.filter_map(|l| {
                Rule::new(&l.expect("string"))
            }).collect();
            let rules = Rules { rules: rules };
            return (state, rules)
        }
    }
    panic!("Shouldn't get here");
}

fn main() {
    let (mut state, rules) = get_state_and_rules();
    println!("State: {:?}", state);
    state.iterate_times(&rules, 20);
    println!("State: {:?}", state);
}
