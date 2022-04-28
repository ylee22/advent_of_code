use std::io::{self, BufRead};

struct Position {
    depth: i32,
    horizontal: i32,
}

impl Position {
    fn new() -> Self {
        return Position{depth: 0, horizontal: 0}
    }

    fn update_horizontal(& mut self, h: i32) {
        self.horizontal = self.horizontal + h;
    }

    fn update_depth_down(& mut self, d: i32) {
        self.depth = self.depth + d;
    }

    fn update_depth_up(& mut self, d: i32) {
        self.depth = self.depth - d;
    }
}

fn parse_lines(p: & mut Position) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {

        let u = line.unwrap();
        let mut l = u.split_whitespace();

        match l.next().unwrap() {
            "forward" => p.update_horizontal(l.next().unwrap().parse::<i32>().unwrap()),
            "down" => p.update_depth_down(l.next().unwrap().parse::<i32>().unwrap()),
            "up" => p.update_depth_up(l.next().unwrap().parse::<i32>().unwrap()),
            _ => panic!("no input")
        }
    }
}

fn main() {
    let mut pos = Position::new();
    parse_lines(& mut pos);
    println!("depth: {}, horizontal: {}", pos.depth, pos.horizontal);
    println!("depth x horizontal: {}", pos.depth*pos.horizontal)
}

#[cfg(test)]
mod tests {
    use crate::Position;

    #[test]
    fn test_update_horizontal() {
        let mut p = Position::new();
        p.update_horizontal(5);
        assert_eq!(p.horizontal, 5);
    }

    #[test]
    fn test_update_depth_positive() {
        let mut p = Position::new();
        p.update_depth_down(5);
        assert_eq!(p.depth, 5);
    }

    #[test]
    fn test_update_depth_negative() {
        let mut p = Position::new();
        p.update_depth_up(5);
        assert_eq!(p.depth, -5);
    }
}