use std::fs;
use std::str::Chars;
use std::fmt::{Formatter,Display,Result};

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Display for Node {
    // `f` is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> Result {
        let children = self.children.iter().map(|c| c.value()).collect::<Vec<u32>>();
        write!(f, "children {:?}, metadata: {:?}",
               children , self.metadata)
    }
}

impl Node {
    // fn format_node(&self, d: usize) -> String {
    //     let mut result = String::new();
    //     let prefix = String::from_utf8(vec![b' '; d * 2]).unwrap();
    //     result.push_str(&prefix);
    //     result.push_str(&format!("metadata: {:?}", self.metadata));
    //     result.push('\n');
    //     result.push_str(&prefix);
    //     result.push_str(&format!("children: {}", self.children.len()));
    //     result.push('\n');
    //     for child in &self.children {
    //         result.push_str(&child.format_node(d + 1));
    //     }
    //     return result;
    // }
    // fn formatted(&self) -> String {
    //     return self.format_node(0)
    // }
    fn value(&self) -> u32 {
        if self.children.len() == 0 {
            return self.metadata.iter().fold(0, |sum,m| { 
                sum + m
            });
        } else {
            return self.metadata.iter().fold(0, |sum,m| {
                if *m == 0 {
                    return sum;
                } else if *m <= self.children.len() as u32 {
                    let child = &self.children[(*m - 1) as usize];
                    let value = child.value(); 
                    return sum + value;
                } else {
                    return sum;
                }
            });
        }
    }
}

fn get_number(string: &mut Chars) -> Option<u32> {
    let mut result = None;
    let mut text = String::new();
    while let Some(c) = string.next() {
        if c.is_whitespace() {
            let parse = text.parse::<u32>();
            if let Ok(value) = parse {
                result = Some(value);
            } else {
                assert!(false);
            }
            break;
        } else {
            text.push(c)
        }
    }
    return result
}

fn get_node(string: &mut Chars) -> Option<Node> {
    if let Some(child_count) = get_number(string) {
        if let Some(metadata_count) = get_number(string) {
            let mut children = Vec::new();
            for _i in 0..child_count {
                if let Some(child) = get_node(string) {
                    children.push(child);
                } else {
                    return None;
                }
            }
            let mut metadata = Vec::new();
            for _i in 0..metadata_count {
                metadata.push(get_number(string).unwrap())
            }
            let node = Node { children: children, metadata: metadata };
            return Some(node);
        }
    }
    return None;
}

fn build_tree() -> Option<Node> {
    let string = fs::read_to_string("input").unwrap();
    let mut chars = string.chars();
    get_node(&mut chars)
}

fn main() {
    let option = build_tree();
    if let Some(tree) = option {
        let sum = tree.value();
        println!("{:?}", sum);
    }
}
