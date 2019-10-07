use std::collections::HashMap;
use std::env;

struct RecipesState {
    recipes: Vec<i32>,
    elves: Vec<i32>
}

impl RecipesState {
    fn contains(&self, sequence: &Vec<i32>) -> Option<usize> {
        if self.recipes.len() < sequence.len() {
            return None;
        }
        for i in 0..sequence.len() {
            if self.recipes[..self.recipes.len() - (i + 1)].ends_with(&sequence[..]) {
                return Some(self.recipes.len() - sequence.len() - i - 1)
            }
        }
        None
    }
}

fn recipes_from_sum(sum: i32) -> Vec<i32> {
    let mut scores: Vec<i32> = Vec::new();
    let mut n = sum;
    while n > 9 {
        scores.push(n % 10);
        n /= 10;
    }
    scores.push(n);
    scores.reverse();
    scores
}

fn test_recipes(state: &mut RecipesState) {
    let mut sum: i32 = 0;
    for elf in &state.elves {
        sum += state.recipes[*elf as usize];
    }
    state.recipes.append(&mut recipes_from_sum(sum));
    for elf in &mut state.elves {
        let advance = 1 + state.recipes[*elf as usize];
        *elf = (*elf + advance) % state.recipes.len() as i32;
    }
}

fn _format_recipes(state: &RecipesState) -> String {
    let mut string = String::new();
    let mut map: HashMap<i32,(char,char)> = HashMap::new();
    let mut wraps: Vec<(char,char)> = vec![('(',')'),('[',']')];
    for elf in &state.elves {
        map.insert(*elf, wraps.pop().unwrap());
    }
    for (idx, recipe) in state.recipes.iter().enumerate() {
        let wrap = map.get(&(idx as i32)).unwrap_or(&(' ',' '));
        string.push_str(&format!("{}{}{}", wrap.0, recipe, wrap.1).to_string());
    }
    string
}

fn _print_recipes(state: &RecipesState) {
    println!("{}", _format_recipes(state));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        std::process::exit(-1);
    }

    let mut state = RecipesState { recipes: vec![3,7], elves: vec![0,1] };
    match &*args[1] {
        "1" => {
            let spinup = 554401;
            let range = 10;
            let mut step = 0;
            while state.recipes.len() < spinup + range {
                if step % 100 == 0 {
                    println!("Done {} iterations", step);
                }
                test_recipes(&mut state);
                step += 1;
            }
            let last: Vec<String> = state.recipes[spinup..spinup+range].iter().map(|m| m.to_string()).collect();
            println!("{}", last.join(""));
        },
        "2" => {
            let target = 554401;
            let sequence = recipes_from_sum(target);
            let mut step = 0;
            loop {
                if step % 100000 == 0 {
                    println!("Done {} iterations", step);
                }
                if let Some(idx) = state.contains(&sequence) {
                    println!("{} first appears after {} recipes", target, idx);
                    break;
                }
                test_recipes(&mut state);
                step += 1;
            }
        },
        _ => {}
    }
}
