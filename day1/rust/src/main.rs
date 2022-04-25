use std::io::{self, BufRead};

fn delta_depth(depths: &Vec<i32>) -> i32 {
    let mut increased = 0;
    let mut curr = 0;
    for i in 0..depths.len() - 2 {
        if i == 0 {
            curr = depths[i..i+3].iter().sum();
        }

        if i > 0 {
            let prev = curr;
            curr = depths[i..i+3].iter().sum();
            if curr > prev {
                increased = increased + 1
            }
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

    let counts: i32 = delta_depth(&d);
    println!("{}", counts)
}

#[cfg(test)]
mod tests {
    use crate::delta_depth;
    use quickcheck as qc;
    use proptest::prelude::*;

    #[test]
    fn test_parse_lines() {
        let test = crate::parse_lines();
        assert_eq!(test, vec![1,2,3])
    }

    #[test]
    fn test_delta_depth() {
        let d = Vec::from([1,2,3,4,5]);
        let counts = delta_depth(&d);
        assert_eq!(counts, 2)
    }

    proptest! {
        #[test]
        fn test4(i1 in 0..1000i32, i2 in 0..1000i32, i3 in 0..1000i32, i4 in 0..1000i32) {
            let v = vec![i1,i2,i3,i4];
            let s1 = i1+i2+i3;
            let s2 = i2+i3+i4;
            let t = delta_depth(&v);
            assert_eq!(s2>s1, t==1);
            assert_eq!(s2<=s1, t==0);
        }
    }

    qc::quickcheck! {
        fn test_increasing(i1: i32, i2: i32, i3: i32, i4: i32) -> bool {
            let v = vec![i1,i2,i3,i4];
            let s1 = i1+i2+i3;
            let s2 = i2+i3+i4;
            let t = delta_depth(&v);
            (s2>s1) == (t==1)
        }
    }
    
}