use std::io::{self, BufRead};

struct Position {
    depth: i32,
    horizontal: i32,
}

fn parse_lines(p: & mut Position) -> Vec<i32> {
    let stdin = io::stdin();
    let mut d = Vec::<i32>::new();
    for line in stdin.lock().lines() {
        let mut l = line.unwrap().split_whitespace();
        match l.next().unwrap() {
            "forward" => p.horizontal = p.horizontal + l.next().unwrap().parse::<i32>().unwrap(),
            "down" => p.depth = p.depth + l.next().unwrap().parse::<i32>().unwrap(),
            "up" => p.depth = p.depth - l.next().unwrap().parse::<i32>().unwrap(), 
            _ => panic!("no input")
        }
    }
    return d;
}

fn main() {
    let mut pos = Position{depth:0, horizontal:0};
    parse_lines(& mut pos);
    println!("depth: {}, horizontal: {}", pos.depth, pos.horizontal)
}
