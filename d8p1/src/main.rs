use std::fs;
use std::str::Chars;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn metadata_sum(&self) -> u32 {
        self.metadata.iter().fold(0, |sum,m| sum + m) + self.children.iter().fold(0, {|sum,c| sum + c.metadata_sum()})
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
        let sum = tree.metadata_sum();
        println!("{:?}", sum);
    }
}
