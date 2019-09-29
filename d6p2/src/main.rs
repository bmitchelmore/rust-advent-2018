extern crate regex;

use std::fs::File;
use std::io::{BufRead,BufReader};
use regex::Regex;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Location {
    x: i32,
    y: i32
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Path {
    start: Location,
    end: Location,
    distance: i32
}

#[derive(Debug)]
struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32
}

impl Location {
    fn distance(&self, loc: &Location) -> i32 {
        (self.x - loc.x).abs() + (self.y - loc.y).abs()
    }
}

fn get_locations() -> Vec<Location> {
    let re = Regex::new(r"(\d+), (\d+)").unwrap();
    let file = File::open("input").expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().map(|l| {
        let string = l.expect("string");
        if re.is_match(&string) {
            for cap in re.captures_iter(&string) {
                let x = &cap[1].parse::<i32>().unwrap();
                let y = &cap[2].parse::<i32>().unwrap();
                return Location { x: *x, y: *y };
            }
        }
        panic!("no match for event")
    }).collect()
}

fn get_extents(locs: &Vec<Location>) -> Rect {
    let min_x = locs.iter().map(|l| l.x).min().unwrap();
    let max_x = locs.iter().map(|l| l.x).max().unwrap();
    let min_y = locs.iter().map(|l| l.y).min().unwrap();
    let max_y = locs.iter().map(|l| l.y).max().unwrap();
    Rect { 
        x: min_x, 
        y: min_y, 
        w: max_x - min_x, 
        h: max_y - min_y 
    }
}

fn points_with_range(locs: &Vec<Location>, extents: &Rect, limit: i32) -> usize {
    let mut hash: HashMap<Location,i32> = HashMap::new();
    for x in (extents.x)..(extents.x+extents.w) {
        for y in (extents.y)..(extents.y+extents.h) {
            let cur = Location { x: x, y: y };
            let paths: Vec<Path> = locs.iter().map(|l| Path { start: cur, end: *l, distance: cur.distance(l) }).collect();
            let distance = paths.iter().fold(0, {|acc,p| acc + p.distance});
            if distance < limit {
                hash.entry(cur).or_insert(distance);
            }
        }
    }
    hash.len()
}

fn main() {
    let locs = get_locations();
    let extents = get_extents(&locs);
    let points = points_with_range(&locs, &extents, 10000);
    println!("{:?}", points);
}
