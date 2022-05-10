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

fn epsilon(gamma: &Vec<i8>) -> Vec<i8> {
    let mut ep = Vec::<i8>::new();
    for c in gamma {
        match c {
            1 => ep.push(0),
            0 => ep.push(1),
            _ => panic!("not 0 or 1")
        }
    }
    return ep
}

fn bit_vec_to_int(v: &Vec<i8>) -> i32{
    let mut score: i32 = 0;
    let vsize = v.len() - 1;
    for (i, n) in v.iter().enumerate() {
        let e = (vsize - i) as u32;
        score = score + (*n as i32)*(2 as i32).pow(e);
    }
    return score
}

fn main() {
    let (mut counts, total) = parse_lines();
    let g = gamma(&mut counts, total);
    let b = epsilon(&g);
    let g = bit_vec_to_int(&g);
    let b = bit_vec_to_int(&b);
    println!("{}", g*b)
}


#[cfg(test)]
mod tests {
    use crate::{count_ones, gamma, epsilon, bit_vec_to_int};

    #[test]
    fn test_count_ones_update() {
        let mut counts = vec![0,1,1,1,0];
        count_ones("00111".to_string(), & mut counts);
        assert_eq!(counts, [0,1,2,2,1]);
    }

    #[test]
    fn test_count_ones_initial() {
        let mut counts = Vec::new();
        count_ones("00111".to_string(), & mut counts);
        assert_eq!(counts, vec![0,0,1,1,1])
    }

    #[test]
    fn test_gamma() {
        let counts = vec![0,2,3,0,5];
        let total = 5;
        let g = gamma(&counts, total);
        assert_eq!(g, vec![0,0,1,0,1])
    }

    #[test]
    fn test_epsilon() {
        let b = epsilon(&vec![0,0,1,1,1]);
        assert_eq!(b, vec![1,1,0,0,0])
    }

    #[test]
    fn test_bit_vec_to_int() {
        let v: Vec<i8> = vec![1,0,1];
        let s = bit_vec_to_int(&v);
        assert_eq!(5, s);
        let v: Vec<i8> = vec![1,0,0];
        let s = bit_vec_to_int(&v);
        assert_eq!(4, s);
        let v: Vec<i8> = vec![1,1,1];
        let s = bit_vec_to_int(&v);
        assert_eq!(7, s)
    }
}