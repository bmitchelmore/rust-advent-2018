extern crate regex;

use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;
use std::fmt;

#[derive(Debug,Copy,Clone)]
struct Pot {
    has_plant: bool
}

#[derive(Debug)]
struct State {
    zero: u64,
    pots: Vec<Pot>
}

#[derive(Debug,Copy,Clone)]
struct Rule {
    state: [bool; 5],
    result: bool
}

#[derive(Debug)]
struct Rules {
    rules: Vec<Rule>
}

impl Rules {
    fn len(&self) -> usize {
        self.rules.len()
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = std::iter::repeat(".").take(20 - self.zero as usize).collect::<String>();
            write!(f, "{}", prefix).expect("could not write string");
        for pot in &self.pots {
            write!(f, "{}", pot).expect("could not write string");
        }
        Ok(())
    }
}

impl fmt::Display for Pot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_bool(f, self.has_plant);
        Ok(())
    }
}

fn str_for_pot_bool(b: bool) -> &'static str {
    match b {
        true => "#",
        false => "."
    }
}

fn str_for_pot(p: Pot) -> &'static str {
    str_for_pot_bool(p.has_plant)
}

fn str_for_optional_pot(p: Option<Pot>) -> &'static str {
    match p {
        None => "-",
        Some(pot) => str_for_pot(pot)
    }
}

fn write_bool(f: &mut fmt::Formatter, b: bool) {
    write!(f, "{}", str_for_pot_bool(b)).expect("could not write string");
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for val in &self.state {
            write_bool(f, *val);
        }
        write!(f, " => ").expect("could not write string");
        write_bool(f, self.result);
        Ok(())
    }
}

impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let count = self.rules.len();
        for (i, rule) in self.rules.iter().enumerate() {
            write!(f, "{}", &rule).expect("could not write string");
            if i != (count - 1) {
                write!(f, "\n").expect("could not write string");
            }
        }
        Ok(())
    }
}

impl State {
    fn new(s: &str) -> State {
        let cap = s.len();
        let mut vec = Vec::with_capacity(cap);
        for c in s.chars() {
            let has_plant = c == '#';
            vec.push(Pot { has_plant: has_plant });
        }
        State { zero: 0, pots: vec }
    }
    fn plant_count(&self) -> i64 {
        let mut count = 0;
        for (i, pot) in self.pots.iter().enumerate() {
            let actual_idx = i as i64 - self.zero as i64;
            if pot.has_plant {
                count += actual_idx;
            }
        }
        count
    }
    fn plant_indices(&self) -> Vec<i64> {
        let mut indices: Vec<i64> = Vec::new();
        let mut count = 0;
        for (i, pot) in self.pots.iter().enumerate() {
            let actual_idx = i as i64 - self.zero as i64;
            if pot.has_plant {
                indices.push(actual_idx);
            }
        }
        indices
    }
    fn current_pot_at(&self, idx: i64) -> Option<Pot> {
        let actual_idx = idx + self.zero as i64;
        if actual_idx < 0 {
            return None
        }

        let actual_idx = actual_idx as usize;
        if actual_idx > self.pots.len() - 1 {
            return None
        } 

        let pot = self.pots[actual_idx];
        Some(pot)
    }
    fn next_pot_at(&self, idx: i64, rules: &Rules) -> Option<(Pot, Rule)> {
        let current = self.current_pot_at(idx);
        let current_val = current.unwrap_or(Pot { has_plant: false });
        let prev2 = self.current_pot_at(idx - 2);
        let prev2_val = prev2.unwrap_or(Pot { has_plant: false });
        let prev1 = self.current_pot_at(idx - 1);
        let prev1_val = prev1.unwrap_or(Pot { has_plant: false });
        let next1 = self.current_pot_at(idx + 1);
        let next1_val = next1.unwrap_or(Pot { has_plant: false });
        let next2 = self.current_pot_at(idx + 2);
        let next2_val = next2.unwrap_or(Pot { has_plant: false });
        for rule in &rules.rules {
            if rule.state[0] == prev2_val.has_plant &&
                rule.state[1] == prev1_val.has_plant &&
                rule.state[2] == current_val.has_plant &&
                rule.state[3] == next1_val.has_plant &&
                rule.state[4] == next2_val.has_plant {
                let replacement = Pot { has_plant: rule.result };
                if rule.result && !current_val.has_plant {
                    return Some((replacement, *rule))
                } else if !rule.result && current_val.has_plant {
                    return Some((replacement, *rule))
                } else {
                    return None
                }
            }
        }
        None
    }
    fn replace_pot_at(&mut self, idx: i64, p: Pot) {
        let actual_idx = idx + self.zero as i64;
        if actual_idx < 0 {
            let grow = actual_idx.abs();
            self.pots.insert(0, p);
            self.zero += grow as u64;
            for _i in 1..grow {
                self.pots.insert(0, Pot { has_plant: false });
            }
            return
        }

        let actual_idx = actual_idx as usize;
        if actual_idx > self.pots.len() - 1 {
            let grow = actual_idx - (self.pots.len() - 1);
            for _i in 1..grow {
                self.pots.push(Pot { has_plant: false });
            }
            self.pots.push(p);
            return
        } 

        self.pots[actual_idx] = p
    }
    fn iterate_times(&mut self, rules: &Rules, n: u32) {
        for i in 0..n {
            println!("{}: {}", i, self);
            self.iterate(rules);
        }
    }
    fn iterate(&mut self, rules: &Rules) {
        let len = self.pots.len() as i64;
        let leeway = 2;
        let start = 0 - leeway;
        let end = len + leeway;
        let mut changes: Vec<(i64, Pot, Rule)> = Vec::new();
        for i in start..end {
            let next = self.next_pot_at(i, rules);
            match next {
                Some((pot, rule)) => changes.push((i, pot, rule)),
                None => continue
            };
        }
        for (i,pot,rule) in changes.iter() {
             self.replace_pot_at(*i, *pot);
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
    println!("Rules");
    println!("==========");
    println!("{}", rules);
    println!("");
    println!("{}", state);
    for i in 0..20 {
        state.iterate(&rules);
    }

    let count = state.plant_count();
    println!("Plant Count After 20 Generations: {}", count);

    for i in 0..180 {
        state.iterate(&rules);
    }
    let indices = state.plant_indices();
    println!("Indices after 200 Generations: {:?}", indices);
    let updated: Vec<i64> = indices.iter().map(|i| { i + (50000000000 - 200) }).collect();
    println!("Translated forward to generation 50,000,000,000: {:?}", updated);
    let sum: i64 = updated.iter().sum();
    println!("Sum for Generation 50,000,000,000: {}", sum);
}
