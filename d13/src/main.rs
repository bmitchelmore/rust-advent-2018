use std::env;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;
use std::mem;

fn read_map(name: &String) -> Vec<Vec<Option<Track>>> {
    let mut cart_id = 0;
    let file = File::open(name).expect("File not found");
    let buf = BufReader::new(file);
    let map: Vec<Vec<Option<Track>>> = buf.lines().map(|l| {
        l.unwrap().chars().map(|c| {
            match c {
                '+' => Some(Track { kind: Kind::Intersection, carts: vec![] }),
                '-' => Some(Track { kind: Kind::LeftRight, carts: vec![] }),
                '|' => Some(Track { kind: Kind::UpDown, carts: vec![] }),
                '/' => Some(Track { kind: Kind::DownRight, carts: vec![] }),
                '\\' => Some(Track { kind: Kind::LeftDown, carts: vec![] }),
                '<' => { 
                    cart_id += 1;
                    Some(Track { kind: Kind::LeftRight, carts: vec![ Cart { id: cart_id, dir: Direction::Left, turn: Turn::Left } ] })
                },
                '>' => { 
                    cart_id += 1;
                    Some(Track { kind: Kind::LeftRight, carts: vec![ Cart { id: cart_id, dir: Direction::Right, turn: Turn::Left } ] })
                },
                '^' => { 
                    cart_id += 1;
                    Some(Track { kind: Kind::UpDown, carts: vec![ Cart { id: cart_id, dir: Direction::Up, turn: Turn::Left } ] })
                },
                'v' => { 
                    cart_id += 1;
                    Some(Track { kind: Kind::UpDown, carts: vec![ Cart { id: cart_id, dir: Direction::Down, turn: Turn::Left } ] })
                },
                _ => None
            }
        }).collect()
    }).collect();
    map
}

fn format_map(map: &Vec<Vec<Option<Track>>>) -> String {
    let ss: Vec<String> = map.iter().map(|ts| { 
        let s: String = ts.into_iter().map(|t| {
            match t {
                None => " ",
                Some(track) => {
                    if track.contains_crash() {
                        "X"
                    } else if let Some(cart) = track.carts.first() {
                        match cart.dir {
                            Direction::Up => "^",
                            Direction::Down => "v",
                            Direction::Left => "<",
                            Direction::Right => ">"
                        }
                    } else {
                        match track.kind {
                            Kind::Intersection => "+",
                            Kind::UpDown => "|",
                            Kind::LeftRight => "-",
                            Kind::DownRight => "/",
                            Kind::LeftDown => "\\"
                        }
                    }
                }
            }
        }).collect();
        s
    }).collect();
    ss.join("\n")
}

fn print_map(map: &Vec<Vec<Option<Track>>>) {
    println!("{}", format_map(map))
}

// fn find_first_crash(map: &Vec<Vec<Option<Track>>>) -> Option<(usize, usize)> {
//     for (i, row) in map.iter().enumerate() {
//         for (j, cell) in row.iter().enumerate() {
//             match cell {
//                 Some(track) if track.contains_crash() => return Some((i, j)),
//                 _ => ()
//             }
//         }
//     }
//     None
// }

fn get_first_cart_position(map: &Vec<Vec<Option<Track>>>) -> Option<(usize, usize)> {
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            match cell {
                Some(track) if track.carts.len() > 0 => return Some((i, j)),
                _ => ()
            }
        }
    }
    None
}

fn get_cart_count(map: &Vec<Vec<Option<Track>>>) -> usize {
    let mut count = 0;
    for row in map {
        for cell in row {
            match cell {
                Some(track) if track.carts.len() > 0 => {
                    count += track.carts.len()
                },
                _ => ()
            }
        }
    }
    count
}

#[derive(Copy,Clone,Debug)]
enum Turn {
    Left,
    Straight,
    Right
}

impl Turn {
    fn next(&self) -> Turn {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Copy,Clone,Debug)]
enum Direction {
    Up,  
    Down,
    Left,
    Right
}

#[derive(Copy,Clone,Debug)]
struct Cart {
    id: i32,
    dir: Direction,
    turn: Turn
}

#[derive(Copy,Clone,Debug)]
enum Kind {
    Intersection,
    UpDown,
    LeftRight,
    DownRight,
    LeftDown
}

#[derive(Debug)]
struct Track {
    kind: Kind,
    carts: Vec<Cart>
}

impl Track {
    fn contains_crash(&self) -> bool {
        self.carts.len() > 1
    }
    fn adding_cart(&self, cart: &Cart) -> Track {
        let mut updated = *cart;
        match self.kind {
            Kind::Intersection => {
                match (cart.turn, cart.dir) {
                    (Turn::Straight, _) => (),
                    (Turn::Left, Direction::Up) => {
                        updated.dir = Direction::Left;
                    },
                    (Turn::Left, Direction::Down) => {
                        updated.dir = Direction::Right;
                    },
                    (Turn::Left, Direction::Left) => {
                        updated.dir = Direction::Down;
                    },
                    (Turn::Left, Direction::Right) => {
                        updated.dir = Direction::Up;
                    },
                    (Turn::Right, Direction::Up) => {
                        updated.dir = Direction::Right;
                    },
                    (Turn::Right, Direction::Down) => {
                        updated.dir = Direction::Left;
                    },
                    (Turn::Right, Direction::Left) => {
                        updated.dir = Direction::Up;
                    },
                    (Turn::Right, Direction::Right) => {
                        updated.dir = Direction::Down;
                    }
                };
                updated.turn = cart.turn.next();
            },
            Kind::DownRight => { // `/`
                match cart.dir {
                    Direction::Right => {
                        updated.dir = Direction::Up;
                    },
                    Direction::Left => {
                        updated.dir = Direction::Down;
                    },
                    Direction::Down => {
                        updated.dir = Direction::Left;
                    },
                    Direction::Up => {
                        updated.dir = Direction::Right;
                    }
                }
            },
            Kind::LeftDown => { // `\`
                match cart.dir {
                    Direction::Right => {
                        updated.dir = Direction::Down;
                    },
                    Direction::Left => {
                        updated.dir = Direction::Up;
                    },
                    Direction::Down => {
                        updated.dir = Direction::Right;
                    },
                    Direction::Up => {
                        updated.dir = Direction::Left;
                    }
                }
            }
            _ => ()
        }
        let mut carts: Vec<Cart> = self.carts.iter().cloned().collect();
        carts.push(updated);
        Track { kind: self.kind, carts: carts }
    }
    fn removing_cart(&self, cart: &Cart) -> Track {
        let mut carts: Vec<Cart> = self.carts.iter().cloned().collect();
        let index = carts.iter().position(|c| c.id == cart.id).unwrap();
        carts.remove(index);
        Track { kind: self.kind, carts: carts }
    }
    fn clear_carts(&self) -> Track {
        Track { kind: self.kind, carts: vec![] }
    }
}

fn perform_tick(map: &mut Vec<Vec<Option<Track>>>) -> Vec<(usize,usize)> {
    let mut done: HashMap<i32,bool> = HashMap::new();
    let mut crashes: Vec<(usize, usize)> = Vec::new();
    let height = map.len();
    let width = map.iter().map(|a| { a.len() }).max().unwrap();
    for i in 0..height {
        for j in 0..width {
            let track = &map[i][j];
            if let Some(track) = track { 
            if let Some(cart) = track.carts.first() {
            let cart_id = cart.id;
            if !done.contains_key(&cart_id)  {
                let idxs = match cart.dir {
                    Direction::Up => {
                        (i-1, j)
                    },
                    Direction::Down => {
                        (i+1, j)
                    },
                    Direction::Right => {
                        (i, j+1)
                    },
                    Direction::Left => {
                        (i, j-1)
                    }
                };
                let current_replacement = Some(map[i][j].as_ref().unwrap().removing_cart(cart));
                let adjacent_replacement = Some(map[idxs.0][idxs.1].as_ref().unwrap().adding_cart(cart));
                mem::replace(&mut map[idxs.0][idxs.1], adjacent_replacement);
                mem::replace(&mut map[i][j], current_replacement);
                if map[i][j].as_ref().unwrap().contains_crash() { 
                    crashes.push((i,j));
                    clear_crash(map, (i,j));
                }
                if map[idxs.0][idxs.1].as_ref().unwrap().contains_crash() {
                    crashes.push(idxs);
                    clear_crash(map, idxs);
                }
                done.insert(cart_id, true);
            }}}
        }
    }
    crashes
}

fn clear_crash(map: &mut Vec<Vec<Option<Track>>>, idxs: (usize,usize)) {
    let current_replacement = Some(map[idxs.0][idxs.1].as_ref().unwrap().clear_carts());
    mem::replace(&mut map[idxs.0][idxs.1], current_replacement);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let fallback = "input".to_string();
    let file = if args.len() > 1 { &args[1] } else { &fallback };
    let mut map = read_map(file);
    let mut crashes: Vec<(usize,usize)> = vec![];
    let mut carts = get_cart_count(&map);
    // print_map(&map);

    // Part 1
    while crashes.len() == 0 {
        crashes = perform_tick(&mut map);
        carts -= crashes.len() * 2;
    }
    if crashes.len() > 0 {
        if let Some(crash) = crashes.first() {
            println!("First Crash: {},{}", crash.1, crash.0);
        }
    }
    // Part 2
    map = read_map(file);
    carts = get_cart_count(&map);
    while carts > 1 {
        let crashes = perform_tick(&mut map);
        let subtrahend = crashes.len() * 2;
        carts -= subtrahend;
        for crash in crashes {
            println!("Crash: {},{}", crash.1, crash.0);
        }
    }
    if let Some(idxs) = get_first_cart_position(&map) {
        println!("Last Cart: {},{}", idxs.1, idxs.0);
    } else {
        println!("No carts remaining!");
    }
}
