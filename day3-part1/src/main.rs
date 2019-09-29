use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead,BufReader};

#[derive(Debug)]
struct Claim {
    id: i32,
    pt: Point,
    sz: Size
}

#[derive(Debug)]
#[derive(Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}

#[derive(Debug)]
struct Size {
    wd: i32,
    ht: i32
}

fn get_claims() -> Vec<Claim> {
    let file = File::open("input").expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().map(|l| { 
        let string = l.expect("string");
        let mut parts = string.split_whitespace();
        let id_str = parts.next().unwrap();
        parts.next();
        let point = parts.next().unwrap();
        let size = parts.next().unwrap();
        let claim_id = id_str.trim_start_matches('#').parse::<i32>().unwrap();
        let points: Vec<i32> = point.trim_end_matches(':').split(',').map(|n| n.parse::<i32>().unwrap()).collect();
        let (x, y) = (points[0], points[1]);
        let sizes: Vec<i32> = size.split('x').map(|n| n.parse::<i32>().unwrap()).collect();
        let (w, h) = (sizes[0], sizes[1]);
        return Claim { id: claim_id, pt: Point { x: x, y: y }, sz: Size { wd: w, ht: h } }
    }).collect()
}

fn main() {
    let claims = get_claims();
    let mut hash: HashMap<Point,i32> = HashMap::new();
    for claim in claims {
        for i in 0..claim.sz.wd {
            for j in 0..claim.sz.ht {
                let pt = Point { x: claim.pt.x + i, y: claim.pt.y + j };
                let entry = hash.entry(pt).or_insert(0);
                *entry += 1;
            }
        }
    }
    let conflicts: HashMap<Point,i32> = hash.into_iter().filter(|(_k, v)| *v >= 2).collect();
    println!("Number of conflicts: {}", conflicts.len());
}
