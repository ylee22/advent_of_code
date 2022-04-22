use std::io::{self, BufRead};

fn delta_depth(depths: Vec<i32>) -> i32 {
    let mut increased = 0;
    for i in 1..depths.len() {
        if depths[i] > depths[i-1] {
            increased = increased + 1
        }
    }
    return increased;
}

fn parse_lines() -> Vec<i32> {
    let stdin = io::stdin();
    let mut d = Vec::<i32>::new();
    for line in stdin.lock().lines() {
        let l = line.unwrap().parse::<i32>().unwrap();
        d.push(l)
    }
    return d;
}

fn main() {

    let d: Vec<i32> = parse_lines();

    let counts: i32 = delta_depth(d);
    println!("{}", counts)
}

#[cfg(test)]
mod tests {
    use crate::delta_depth;


    #[test]
    fn test_parse_lines() {
        let test = crate::parse_lines();
        assert_eq!(test, vec![1,2,3])
    }

    #[test]
    fn test_delta_depth() {
        let d = Vec::from([1,2,3]);
        let counts = delta_depth(d);
        assert_eq!(counts, 2)
    }
}