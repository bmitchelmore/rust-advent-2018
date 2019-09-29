use std::fs::File;
use std::io::{BufRead,BufReader};

fn get_boxes() -> Vec<String> {
    let file = File::open("input").expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("no line")).collect()
}

fn main() {
    let mut boxes = get_boxes();
    boxes.sort();
    let first = boxes.iter();
    let mut second = boxes.iter();
    second.next();
    let it = first.zip(second);
    for (first, second) in it {
        let mut diff = 0;
        let mut common = String::new();
        for (first, second) in first.chars().zip(second.chars()) {
            if first != second {
                diff += 1
            } else {
                common.push(first)
            }
        }
        if diff == 1 {
            println!("common: {}", common);
        }
    }
}
