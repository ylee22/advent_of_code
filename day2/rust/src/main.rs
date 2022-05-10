use std::io::{self, BufRead};

#[derive(Debug)]
struct Position {
    depth: i32,
    horizontal: i32,
    aim: i32
}

impl Position {
    fn new() -> Self {
        return Position{depth: 0, horizontal: 0, aim: 0}
    }

    fn update_horizontal(& mut self, h: i32) {
        self.horizontal = self.horizontal + h;
        self.depth = (self.aim * h) + self.depth;
    }

    fn update_depth_down(& mut self, d: i32) {
        // self.depth = self.depth + d;
        self.aim = self.aim + d;
    }

    fn update_depth_up(& mut self, d: i32) {
        // self.depth = self.depth - d;
        self.aim = self.aim - d;
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
    use quickcheck as qc;

    #[test]
    fn test_update_horizontal() {
        let mut p = Position{horizontal: 0, depth: 5, aim:0};
        p.update_horizontal(5);
        assert_eq!(p.horizontal, 5);
        assert_eq!(p.depth, 5)
    }

    #[test]
    fn test_update_depth_positive() {
        let mut p = Position::new();
        p.update_depth_down(5);
        assert_eq!(p.depth, 5);
        assert_eq!(p.aim, 5)
    }

    #[test]
    fn test_update_depth_negative() {
        let mut p = Position::new();
        p.update_depth_up(5);
        assert_eq!(p.depth, -5);
        assert_eq!(p.aim, -5)
    }

    #[derive(Clone, Debug)]
    enum Command {
        Forward (u8),
        Up (u8),
        Down (u8)
    }

    impl qc::Arbitrary for Command {
        fn arbitrary(g: &mut qc::Gen) -> Self {
            let u = u8::arbitrary(g);
            let cs = [Command::Forward(u), Command::Up(u), Command::Down(u)];
            let c = g.choose(&cs);
            return c.unwrap().clone();
        }
    }

    #[derive(Clone)]
    struct MyVec<T>(Vec<T>);
    impl<T:qc::Arbitrary> qc::Arbitrary for MyVec<T> {
        fn arbitrary(g: &mut qc::Gen) -> Self {
            let u = usize::arbitrary(g);
            let mut v = Vec::new();
            for _ in [0..u] {
                v.push(T::arbitrary(g))
            }
            return MyVec(v);
        }
    }
    
    fn execute(c: &Command, p: &mut Position) {
        match c {
            Command::Forward(u) => p.update_horizontal(*u as i32),
            Command::Up(u) => p.update_depth_up(*u as i32),
            Command::Down(u) => p.update_depth_down(*u as i32)
        }
    }

    fn forward_dist(c: &[Command]) -> i32 {
        let mut horizontal = 0;
        for cmd in c {
            match cmd {
                Command::Forward(u) => horizontal = horizontal + (*u as i32),
                _ => {}
            }
        }
        return horizontal
    }

    qc::quickcheck! {
        fn test_commands(cmds: Vec<Command>) -> bool {
            println!("{}", cmds.len());
            let mut p = Position::new();
            for cmd in &cmds {
                execute(&cmd, &mut p)
            }

            p.horizontal == forward_dist(&cmds[..])
        }
    }

    qc::quickcheck! {
        fn test_aim(cmds: Vec<Command>, forward: u8) -> bool {
            let mut p = Position::new();
            for cmd in &cmds {
                execute(&cmd, &mut p)
            }

            let d_i = p.depth;
            p.update_horizontal(forward as i32);

            d_i + p.aim * (forward as i32) == p.depth
        }
    }
}