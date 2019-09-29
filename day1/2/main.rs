use std::fs::File;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let mut total = 0;
    let mut hash = HashSet::new();
    let mut done = false;
    while !done {
        let file = File::open("input")?;
        for line in BufReader::new(file).lines() {
	    let str = line?;
            let num = &str[1..].parse::<i32>().unwrap();
            if str.starts_with('-') {
                total -= num;
            } else if str.starts_with('+') {
                total += num;
            }
            if hash.contains(&total) {
                println!("found second instance of {}", total);
                done = true;
                break;
	    }
	    hash.insert(total);
        }
    }
    Ok(())
}
