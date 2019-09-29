extern crate chrono;
extern crate regex;
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead,BufReader};
use chrono::{DateTime,Duration,Utc};
use chrono::prelude::*;
use regex::Regex;

#[derive(Debug)]
struct Log {
    date: DateTime<Utc>,
    event: Event,
}

#[derive(Debug)]
struct RawLog {
    date: DateTime<Utc>,
    event: String,
}

#[derive(Debug)]
enum Event {
    StartShift(i32),
    Sleep(i32),
    Wake(i32),
}

fn get_sorted_logs() -> Vec<Log> {
    let re = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)").unwrap();
    let file = File::open("input").expect("File not found");
    let buf = BufReader::new(file);
    let mut logs: Vec<RawLog> = buf.lines().map(|l| { 
        let string = l.expect("string");
        if re.is_match(&string) {
            for cap in re.captures_iter(&string) {
                let y = &cap[1].parse::<i32>().unwrap();
                let m = &cap[2].parse::<u32>().unwrap();
                let d = &cap[3].parse::<u32>().unwrap();
                let h = &cap[4].parse::<u32>().unwrap();
                let min = &cap[5].parse::<u32>().unwrap();
                let dt = Utc.ymd(*y, *m, *d).and_hms(*h, *min, 0);
                let st = &cap[6];
                return RawLog { date: dt, event: st.to_string() };
            }
            panic!("no match for event")
        } else {
            panic!("No match!");
        }
    }).collect();
    logs.sort_unstable_by_key(|l| l.date);
    let mut last_guard_id: Option<i32> = None;
    logs.into_iter().map(|l| {
        let start_re = Regex::new(r"Guard #(\d+) begins shift").unwrap();
        let sleep_re = Regex::new(r"falls asleep").unwrap();
        let wake_re = Regex::new(r"wakes up").unwrap();
        if start_re.is_match(&l.event) {
            for cap in start_re.captures_iter(&l.event) {
                let id = &cap[1].parse::<i32>().unwrap();
                last_guard_id = Some(*id);
                return Log { date: l.date, event: Event::StartShift(*id) }
            }
            panic!("No match!");
        } else if sleep_re.is_match(&l.event) {
            if let Some(id) = last_guard_id {
                return Log { date: l.date, event: Event::Sleep(id) }
            } else {
                panic!("Missing last guard id");
            }
        } else if wake_re.is_match(&l.event) { 
            if let Some(id) = last_guard_id {
                return Log { date: l.date, event: Event::Wake(id) }
            } else {
                panic!("Missing last guard id");
            }
        } else {
            panic!("No match!");
        }
    }).collect()
}

fn main() {
    let logs = get_sorted_logs();
    let mut hash: HashMap<i32,i64> = HashMap::new();
    let mut last_start: Option<DateTime<Utc>> = None;
    for log in &logs {
        match log.event {
            Event::StartShift(_id) => {

            },
            Event::Sleep(_id) => {
                last_start = Some(log.date);
            },
            Event::Wake(id) => {
                if let Some(start) = last_start {
                    let diff = log.date.signed_duration_since(start);
                    let entry = hash.entry(id).or_insert(0);
                    *entry += diff.num_minutes();
                } else {
                    panic!("Invalid wake");
                }
            },
        }
    }
    let mut sleepers: Vec<(&i32,&i64)> = hash.iter().collect();
    sleepers.sort_unstable_by_key(|(_k,v)| *v);
    let (sleepy_guard, sleep_minutes) = sleepers.last().unwrap();
    println!("Sleepiest Guard: {}, Minutes Slept: {}", sleepy_guard, sleep_minutes);
    let mut minutes: HashMap<u32,i32> = HashMap::new();
    let mut last_sleep_start: Option<DateTime<Utc>> = None;
    for log in &logs {
        match log.event {
            Event::Sleep(id) => {
                if id == **sleepy_guard {
                    last_sleep_start = Some(log.date)
                }
            },
            Event::Wake(id) => {
                if id == **sleepy_guard {
                    if let Some(start) = last_sleep_start {
                        let mut mut_start = start;
                        while log.date.signed_duration_since(mut_start).num_minutes() != 0 {
                            let current_minute = mut_start.minute();
                            let entry = minutes.entry(current_minute).or_insert(0);
                            *entry += 1;
                            mut_start = mut_start + Duration::minutes(1);
                        }
                    } else {
                        panic!("Invalid wake");
                    }
                }
            },
            _ => {

            }
        }
    }
    let mut sleep_times: Vec<(&u32,&i32)> = minutes.iter().collect();
    sleep_times.sort_unstable_by_key(|(_k,v)| *v);
    let (sleepiest_time, sleep_count) = sleep_times.last().unwrap();
    println!("Sleepiest Minute: {}, Number of Sleeps: {}", sleepiest_time, sleep_count);
    println!("Result: {}", **sleepy_guard * **sleepiest_time as i32);
}
