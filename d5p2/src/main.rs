use std::fs;
use std::collections::HashSet;

fn get_polymer() -> String {
    fs::read_to_string("input").expect("exists").trim().to_string()
}

fn reduce(polymer: &String) -> String {
    let mut result = String::new();
    let mut iter = polymer.chars().peekable();
    loop {
        if let Some(first) = iter.next() {
            if let Some(second) = iter.peek() {
                if first.is_uppercase() != second.is_uppercase() && first.to_uppercase().to_string() == second.to_uppercase().to_string() {
                    iter.next();
                    continue;
                } else {
                    result.push(first);
                }
            } else {
                result.push(first);
            }
        } else {
            break;
        }
    }
    if result == *polymer {
        println!("original_length: {}", polymer.len());
        println!("result_length: {}", result.len());
        return result;
    } else {
        return reduce(&result);
    }
}

fn get_units(polymer: &String) -> HashSet<char> {
    let mut units: HashSet<char> = HashSet::new();
    for c in polymer.chars() {
        units.insert(c.to_lowercase().next().expect("invalid char"));
    }
    units
}

fn remove_unit(c: char, polymer: &String) -> String {
    let lower = c.to_string();
    let upper = c.to_uppercase().to_string();
    polymer.replace(&lower, "").replace(&upper, "")
}

fn main() {
    let polymer = get_polymer();
    let units = get_units(&polymer);
    let mut smallest: Option<(char,usize)> = None;
    for c in units {
        println!("Checking reduction for {}", c);
        let removed = remove_unit(c, &polymer);
        let reduced = reduce(&removed);
        let len = reduced.len();
        if let Some((_c, small)) = smallest {
            if small > len {
                smallest = Some((c, len));
            }
        } else {
            smallest = Some((c, len));
        }
    }   
    println!("Smallest: {:?}", smallest.expect("Smallest").1);
}
