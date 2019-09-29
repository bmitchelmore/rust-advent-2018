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

impl Rect {
    fn expanded_by(&self, i: i32) -> Rect {
        Rect {
            x: self.x - i,
            y: self.y - i,
            w: self.w + (i * 2),
            h: self.h + (i * 2)
        }
    }
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

fn closest(locs: &Vec<Location>, extents: &Rect) -> HashMap<Location,i32> {
    let mut hash: HashMap<Location,i32> = HashMap::new();
    for x in (extents.x)..(extents.x+extents.w) {
        for y in (extents.y)..(extents.y+extents.h) {
            let cur = Location { x: x, y: y };
            let mut paths: Vec<Path> = locs.iter().map(|l| Path { start: cur, end: *l, distance: cur.distance(l) }).collect();
            paths.sort_by_key(|p| p.distance);
            let closest = &paths.first().unwrap();
            let second = &paths.iter().nth(1).unwrap();
            if closest.distance != second.distance {
                let entry = hash.entry(closest.end).or_insert(0);
                *entry += 1;
            }
        }
    }
    hash
}

fn main() {
    let locs = get_locations();
    let original_extents = get_extents(&locs);
    let expanded_extents = original_extents.expanded_by(25);
    let mut answers: Vec<(&Location,&i32)> = Vec::new();
    let first = closest(&locs, &original_extents);
    let second = closest(&locs, &expanded_extents);
    for (loc, val) in &first {
        let other = second.get(&loc).unwrap();
        if *other == *val {
            answers.push((loc,val));
        }
    }
    answers.sort_unstable_by_key(|(_k,v)| *v);
    let answer = answers.last().unwrap();
    println!("{:?}", answer);
}
