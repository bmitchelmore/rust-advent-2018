// use std::collections::LinkedList

fn mod_sub(a: usize, b: usize, m: usize) -> usize {
    if m == 0 {
        return 0
    } else if b > a {
        let i = m - ((b - a) % m);
        return i;
    } else {
        return a - b;
    }
}

// struct Circle<T> {
//     pub marble: T,
//     pub prev: usize,
//     pub next: usize,
// }

// fn play_game_ll(players: usize, turns: usize) -> Vec<usize> {
//     let mut scores = vec![0; players];
//     let mut circle: LinkedList<usize> = LinkedList::new();
//     circle.ap
//     let mut arena: Arena<Circle<usize>> = Arena::new();
//     let mut circle = arena.alloc(Circle { marble: 0, prev: 0, next: 0 });
//     let mut current_player = 0;
//     for i in 1..=turns {
//         if i % 23 == 0 {
//             if _i in 0..7 {
//                 arena
//                 circle = circle.prev();
//             }
//             scores[current_player] += i + circle.marble;
//             circle = circle.remove();
//         } else {
//             circle = circle.next();
//             circle.append(i);
//         }
//         current_player = (current_player + 1) % players;
//         // let current_percentage = ((10000 as f64 * i as f64 / turns as f64) / 100 as f64) as i32;
//         // if previous_percentage != current_percentage {
//         //     previous_percentage = current_percentage;
//         //     println!("{}%", current_percentage);
//         //     println!("current circle size: {}", circle_len);
//         // }
//     }
//     return scores;
// }

fn play_game(players: usize, turns: usize) -> Vec<usize> {
    let mut scores = vec![0; players];
    let mut circle = Vec::new();
    circle.push(0);
    // vec![0;1];
    let mut current_index = 0;
    let mut current_player = 0;
    let mut circle_len = 1;
    for i in 1..=turns {
        if i % 23 == 0 {
            let remove_index = mod_sub(current_index, 7, circle_len);
            let removed = circle[remove_index];
            println!("removing {} from circle", removed);
            scores[current_player] = i + removed;
            circle.remove(remove_index);
            current_index = remove_index;
            circle_len -= 1;
            println!("{}, {:?}", current_player, scores)
        } else {
            let before_index = (current_index + 1) % circle_len;
            let after_index = (current_index + 2) % circle_len;
            if after_index == 0 {
                circle.push(i);
                current_index = before_index + 1;
            } else {
                circle.insert(after_index, i);
                current_index = after_index;
            }
            circle_len += 1;
        }
        current_player = (current_player + 1) % players;
    }
    return scores;
}

// fn high_score(scores: &Vec<usize>) -> usize {
//     scores.iter().fold(0, |h,c| { 
//         if h > *c {
//             return h;
//         } else {
//             return *c;
//         }
//     })
// }

fn main() {
    // let mut previous_high_score = 0;
    // for i in 1..=5000 {
    //     let scores = play_game(424, i);
    //     let current_high_score = high_score(&scores);
    //     if current_high_score != previous_high_score {
    //         let diff = current_high_score - previous_high_score;
    //         previous_high_score = current_high_score;
    //         println!("high score for {}: {} (-{})", i, current_high_score, diff);
    //     }
    // }
    let scores = play_game(9, 1000);
    // let scores = play_game_ll(424, 7114400);
    println!("scores: {:?}", scores);
    println!("high score: {}", scores.iter().fold(0, |h,c| { 
        if h > *c {
            return h;
        } else {
            return *c;
        }
    }));
}
