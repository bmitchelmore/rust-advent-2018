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
    let mut minutes: HashMap<i32,HashMap<u32,i32>> = HashMap::new();
    let mut last_sleep_start: Option<DateTime<Utc>> = None;
    for log in &logs {
        match log.event {
            Event::Sleep(_id) => {
                last_sleep_start = Some(log.date)
            },
            Event::Wake(id) => {
                if let Some(start) = last_sleep_start {
                    let mut mut_start = start;
                    while log.date.signed_duration_since(mut_start).num_minutes() != 0 {
                        let current_minute = mut_start.minute();
                        let entry = minutes.entry(id).or_insert(HashMap::new()).entry(current_minute).or_insert(0);
                        *entry += 1;
                        mut_start = mut_start + Duration::minutes(1);
                    }
                } else {
                    panic!("Invalid wake");
                }
            },
            _ => {

            }
        }
    }
    let mut sleepiest_minutes: HashMap<i32,(u32,i32)> = HashMap::new();
    for (id, minutes) in minutes {
        let mut sleepiest: Option<(u32,i32)> = None;
        for (minute, count) in minutes {
            if let Some((_current_sleepiest_minute,current_sleepiest_count)) = sleepiest {
                if current_sleepiest_count < count {
                    sleepiest = Some((minute, count));
                }
            } else {
                sleepiest = Some((minute, count));
            }
        }
        if let Some(current_sleepiest) = sleepiest {
            sleepiest_minutes.entry(id).or_insert(current_sleepiest);
        }
    }
    let mut result: Option<(i32,u32,i32)> = None;
    for (id, (minute, count)) in sleepiest_minutes {
        if let Some((_guard_id,_sleep_minute,sleep_count)) = result {
            if sleep_count < count {
                result = Some((id,minute,count))
            }
        } else {
            result = Some((id,minute,count))
        }
    }
    if let Some((id,minute,_count)) = result {
        println!("Result: {}", id * minute as i32);
    }
}
