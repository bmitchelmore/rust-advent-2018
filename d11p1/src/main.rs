fn power_level(x: i32, y: i32, sn: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y + sn;
    power_level *= rack_id;
    power_level = (((power_level % 1000) as f32) / 100 as f32) as i32;
    power_level -= 5;
    power_level
}

// fn print_power_level(x: i32, y: i32, sn: i32) {
//     let power_level = power_level(x, y, sn);
//     println!("Fuel cell at {},{}, grid serial number {}, power level: {}", x, y, sn, power_level);
// }

// fn power_level_for_square(x: i32, y: i32, w: i32, h: i32, sn: i32) -> i32 {
//     let mut total = 0;
//     for i in 0..w {
//         for j in 0..h {
//             total += power_level(x + i, y + j, sn);
//         }
//     }
//     total
// }

// fn examine_grid(mx: i32, my: i32, sn: i32) -> (i32, i32, i32, i32) {
//     let mut largest: (i32, i32, i32, i32) = (0, 0, 1, power_level_for_square(0, 0, 1, 1, sn));
//     for sz in 1..=mx {
//         println!("Checking for size {}", sz);
//         for i in 0..=(mx-sz) {
//             for j in 0..=(my-sz) {
//                 let level = power_level_for_square(i, j, sz, sz, sn);
//                 if level > largest.2 {
//                     largest = (i, j, sz, level);
//                 }
//             }
//         }
//     }
//     largest
// }

struct Grid {
    sz: i32,
    cells: Vec<i32>
}

impl Grid {
    fn power(&self, i: i32, j: i32, sz: i32) -> i32 {
        let mut value = 0;
        for x in i..(i+sz) {
            for y in j..(j+sz) {
                let idx: usize = y as usize + self.sz as usize * x as usize;
                let cell = self.cells[idx];
                value += cell;
            }
        }
        value
    }
    fn largest(&self, sz: i32) -> (i32, i32, i32) {
        let mut largest = (1, 1, 0);
        for i in 0..(self.sz-sz) {
            for j in 0..(self.sz-sz) {
                let power = self.power(i, j, sz);
                if power > largest.2 {
                    largest = (i + 1, j + 1, power);
                }
            }
        }
        largest
    }
}

fn build_grid(sz: i32, sn: i32) -> Grid {
    let mut cells = Vec::new();
    for i in 1..=sz {
        for j in 1..=sz {
            let power = power_level(i, j, sn);
            cells.push(power);
        }
    }
    Grid { sz: sz, cells: cells }
}

fn main() {
    let grid = build_grid(300, 6392);
    let mut largest = (1,1,1,1);
    for i in 1..=300 {
        let value = grid.largest(i);
        if value.2 > largest.3 {
            largest = (value.0, value.1, i, value.2);
        }
        println!("largest as of size {}: {},{} of value {}", i, largest.0, largest.1, largest.2);
    }
    println!("Largest: {},{} with size {}, of value {}", largest.0, largest.1, largest.2, largest.3);
}
