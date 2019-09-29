use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let mut total = 0;
    let file = File::open("input")?;
    for line in BufReader::new(file).lines() {
	let str = line?;
        println!("{}", str);
        let num = &str[1..].parse::<i32>().unwrap();
        if str.starts_with('-') {
            total -= num
        } else if str.starts_with('+') {
            total += num
        }
    }
    println!("total is {}", total);
    Ok(())
}
