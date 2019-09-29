use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead,BufReader,Result};

fn main() -> Result<()> {
    let mut pairs = 0;
    let mut triples = 0;
    let file = File::open("input")?;
    for line in BufReader::new(file).lines() {
        let mut hash: HashMap<char, i32> = HashMap::new();

        let str = line?;
        for c in str.chars() {
            let count = hash.entry(c).or_insert(0);
            *count += 1;
        }
        
        let mut found_pair = false;
        let mut found_triple = false;
        for (_c, count) in &hash {
            if *count == 2 && found_pair == false {
                found_pair = true;
                pairs += 1;
            } else if *count == 3 && found_triple == false {
                found_triple = true;
                triples += 1;
            }
        }
    }
    println!("checksum = {}", pairs * triples);
    Ok(())
}
