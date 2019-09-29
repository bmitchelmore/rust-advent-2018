use std::fs;

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

fn main() {
    let polymer = get_polymer();
    reduce(&polymer);
}
