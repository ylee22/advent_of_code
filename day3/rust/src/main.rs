use std::{io::{self, BufRead}};


fn parse_lines() -> Vec<Vec<u8>> {
    let stdin = io::stdin();

    // store all of the bit vectors
    let mut bit_vecs = Vec::<Vec<u8>>::new();

    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        let char_to_bit = |c: char| {
            match c {
                '1' => 1_u8,
                '0' => 0_u8,
                _ => panic!("value other than 0 or 1")
            }
        };

        let v = line.chars().map(char_to_bit).collect();
        bit_vecs.push(v);

    }
    return bit_vecs
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

fn gamma(counts: &Vec<i32>, total: i32) -> Vec<u8> {
    // todo: half is currently an int that gets rounded, convert to float?
    let half = total / 2;
    let mut gamma = Vec::<u8>::new();
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

fn epsilon(gamma: &Vec<u8>) -> Vec<u8> {
    let mut ep = Vec::<u8>::new();
    for c in gamma {
        match c {
            1 => ep.push(0),
            0 => ep.push(1),
            _ => panic!("not 0 or 1")
        }
    }
    return ep
}

fn bit_vec_to_int(v: &Vec<u8>) -> u32 {
    let mut score: u32 = 0;
    // let vsize = v.len() - 1;
    let two: u32 = 2;

    let power = |(i, n): (usize, &u8)| {
        let e = (v.len() - 1 - i) as u32;
        return (*n as u32)*two.pow(e)
    };

    score = v.iter().enumerate().map(power).sum();

    return score
}

fn filter(v_outer: &mut Vec<Vec<u8>>, carbon: bool) {
    let columns = v_outer[0].len();
    for i in 0 .. columns {

        // find the most common bit for each column
        let counts: f32 = v_outer.iter().map(|v_inner| v_inner[i] as f32).sum();
        let mut common: u8 = 0;

        if counts >= v_outer.len() as f32/2.0 {
            common = 1
        }
        else {
            common = 0
        };

        if carbon {
            match common {
                1 => { common = 0 },
                0 => { common = 1 },
                _ => (),
            }
        };

        // filter
        v_outer.retain(|v_inner| v_inner[i] == common);
        
        if v_outer.len() == 1 {
            return
        }
    }
}


fn main() {
    let bv = parse_lines();
    // let g = gamma(&mut counts, total);
    // let b = epsilon(&g);
    // let g = bit_vec_to_int(&g);
    // let b = bit_vec_to_int(&b);
    // println!("{}", g*b)

    let mut oxygen = bv.clone();
    let mut carbon = bv.clone();

    filter(&mut oxygen, true);
    filter(&mut carbon, false);

    println!("{:?}", oxygen[0]);
    println!("{:?}", carbon[0]);

    let o = bit_vec_to_int(&oxygen[0]);
    let c = bit_vec_to_int(&carbon[0]);
    println!("{}", o*c)
}


#[cfg(test)]
mod tests {
    use crate::{count_ones, gamma, epsilon, bit_vec_to_int, filter};

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
        let v: Vec<u8> = vec![1,0,1];
        let s = bit_vec_to_int(&v);
        assert_eq!(5, s);
        let v: Vec<u8> = vec![1,0,0];
        let s = bit_vec_to_int(&v);
        assert_eq!(4, s);
        let v: Vec<u8> = vec![1,1,1];
        let s = bit_vec_to_int(&v);
        assert_eq!(7, s)
    }

    #[test]
    fn test_filter_oxygen() {
        let mut v = vec![vec![0,1,1], vec![1,1,1], vec![0,0,1]];
        filter(&mut v, true);
        assert_eq!(v[0], vec![0,1,1])
    }

    #[test]
    fn test_filter_carbon() {
        let mut v = vec![vec![0,1,1], vec![1,1,1], vec![0,0,1]];
        filter(&mut v, true);
        assert_eq!(v[0], vec![1,1,1])
    }
}