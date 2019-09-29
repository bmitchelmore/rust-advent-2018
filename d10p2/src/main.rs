extern crate regex;

use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Vel {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Point {
    pos: Pos,
    vel: Vel
}

#[derive(Debug)]
struct Points {
    points: Vec<Point>,
    time: usize
}

#[derive(Debug)]
struct Size {
    wd: i32,
    ht: i32
}

#[derive(Debug)]
struct Rect {
    origin: Pos,
    size: Size
}

impl Size {
    fn area(&self) -> i64 {
        self.ht as i64 * self.wd as i64
    }
}

impl Rect {
    fn point(p: &Point) -> Rect {
        Rect { origin: Pos { x: p.pos.x, y: p.pos.y }, size: Size { wd: 1, ht: 1} }
    }
    fn add(&self, p: &Point) -> Rect {
        let mut minx = self.origin.x;
        let mut maxx = self.origin.x + self.size.wd;
        let mut miny = self.origin.y;
        let mut maxy = self.origin.y + self.size.ht;
        if p.pos.x < minx {
            minx = p.pos.x;
        }
        if p.pos.y < miny {
            miny = p.pos.y;
        }
        if p.pos.x > maxx {
            maxx = p.pos.x + 1;
        }
        if p.pos.y > maxy {
            maxy = p.pos.y + 1;
        }
        Rect { origin: Pos { x: minx, y: miny }, size: Size { wd: maxx - minx, ht: maxy - miny } }
    }
}

impl Points {
    fn extents(&self) -> Rect {
        self.points.iter().fold(Rect::point(self.points.iter().next().unwrap()), |r,p| r.add(p))
    }
    fn area(&self) -> i64 {
        self.extents().size.area()
    }
    fn iterate(&mut self) {
        self.time += 1;
        for i in 0..self.points.len() {
            let pt = &self.points[i];
            let x = pt.pos.x + pt.vel.x;
            let y = pt.pos.y + pt.vel.y;
            self.points[i] = Point { pos: Pos { x, y }, vel: Vel { x: pt.vel.x, y: pt.vel.y } };
        }
    }
    fn reverse(&mut self) {
        self.time -= 1;
        for i in 0..self.points.len() {
            let pt = &self.points[i];
            let x = pt.pos.x - pt.vel.x;
            let y = pt.pos.y - pt.vel.y;
            self.points[i] = Point { pos: Pos { x, y }, vel: Vel { x: pt.vel.x, y: pt.vel.y } };
        }
    }
    fn draw(&self) {
        let extents = self.extents();
        let mut hash: HashMap<i32,HashMap<i32,char>> = HashMap::new();
        for i in extents.origin.y..(extents.origin.y+extents.size.ht) {
            let row = hash.entry(i).or_insert(HashMap::new());
            for j in extents.origin.x..(extents.origin.x+extents.size.wd) {
                row.entry(j).or_insert('.');
            }
        }
        for p in &self.points {
            for i in extents.origin.y..(extents.origin.y+extents.size.ht) {
                for j in extents.origin.x..(extents.origin.x+extents.size.wd) {
                    if p.pos.y == i && p.pos.x == j {
                        let row = hash.entry(i).or_default();
                        let cell = row.entry(j).or_default();
                        *cell = '#';
                    }
                }
            }
        }
        for i in extents.origin.y..(extents.origin.y+extents.size.ht) {
            for j in extents.origin.x..(extents.origin.x+extents.size.wd) {
                let row = hash.entry(i).or_default();
                let cell = row.entry(j).or_default();
                print!("{}", cell);
            }
            println!("");
        }
    }
}

fn get_points() -> Points {
    let re = Regex::new(r"position=<\s*(-?[0-9]+)\s*,\s*(-?[0-9]+)\s*> velocity=<\s*(-?[0-9]+)\s*,\s*(-?[0-9]+)\s*>").unwrap();
    let file = File::open("input").expect("File not found");
    let buf = BufReader::new(file);
    let points = buf.lines().map(|l| {
        let string = l.expect("string");
        if re.is_match(&string) {
            for cap in re.captures_iter(&string) {
                let posx = cap[1].parse::<i32>().unwrap();
                let posy = cap[2].parse::<i32>().unwrap();
                let velx = cap[3].parse::<i32>().unwrap();
                let vely = cap[4].parse::<i32>().unwrap();
                return Point { pos: Pos { x: posx, y: posy }, vel: Vel { x: velx, y: vely }};
            }
        }
        panic!("no match for event")
    }).collect();
    Points { points, time: 0 }
}

fn main() {
    let mut points = get_points();
    let mut area = points.area();
    loop {
        points.iterate();
        let updated = points.area();
        if updated > area {
            points.reverse();
            break;
        }
        area = updated;
    }
    points.draw();
    println!("{}", points.time);
}
