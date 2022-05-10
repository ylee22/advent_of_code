use std::{io::{self, BufRead}};


fn parse_lines() -> (Vec<i32>, i32) {
    let stdin = io::stdin();

    let mut counts = Vec::<i32>::new();

    let mut total = 0;

    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        total = total + 1;
        count_ones(line, &mut counts)
    }
    return (counts, total)
}

fn count_ones(line: String, counts: &mut Vec<i32>) {
    for (i, c) in line.chars().enumerate() {
        if counts.len() > i {
            if c == '1' {
                counts[i] = counts[i] + 1;
            }
        }
        else {
            if c == '1' {
                counts.push(1)
            }
            else {
                counts.push(0)
            }            
        }
    }
}

fn gamma(counts: &Vec<i32>, total: i32) -> Vec<i8> {
    // todo: half is currently an int that gets rounded, convert to float?
    let half = total / 2;
    let mut gamma = Vec::<i8>::new();
    for c in counts {
        if c > &half {
            gamma.push(1)
        }
        else {
            gamma.push(0)
        }
    }
    return gamma
}

fn beta(gamma: &Vec<i8>) -> Vec<i8> {
    let mut beta = Vec::<i8>::new();
    for c in gamma {
        match c {
            1 => beta.push(0),
            0 => beta.push(1),
            _ => panic!("not 0 or 1")
        }
    }
    return beta
}

fn main() {
    let (mut counts, total) = parse_lines();
    let g = gamma(&mut counts, total);
    let b = beta(&g);
    
}


#[cfg(test)]
mod tests {
    use crate::{count_ones, gamma, beta};

    #[test]
    fn test_count_ones() {
        let mut counts = vec![0,1,1,1,0];
        count_ones("00111".to_string(), & mut counts);
        assert_eq!(counts[0], 0);
        assert_eq!(counts[1], 1);
        assert_eq!(counts[2], 2);
        assert_eq!(counts[3], 2);
        assert_eq!(counts[4], 1);
    }

    #[test]
    fn test_gamma() {
        let counts = vec![0,2,3,0,5];
        let total = 5;
        let g = gamma(&counts, total);
        assert_eq!(g, vec![0,0,1,0,1])
    }

    #[test]
    fn test_beta() {
        let b = beta(&vec![0,0,1,1,1]);
        assert_eq!(b, vec![1,1,0,0,0])
    }
}