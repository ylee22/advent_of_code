// unique:
// 2 segments: 1 (c, f)
// 3 segments: 7 (a, c, f)
// 4 segments: 4 (b, c, d, f)
// 5 segments: 2, 3, 5
// 6 segments: 6, 9, 0
// 7 segments: 8 (a, b, c, d, e, f, g)
// 1 - 7 = a*
// if all 1 segment in 5 segment number = 3
// 3 - 7 = d, g -> (d, g) and 4 = d*
// 4 - 1 - d = b*
// if 5 segment and b = 5
// the remaining 5 segment = 2
// 2 - 3 = e*, remaining segment in 3 = f*
// 1 - f = c*
// 0 - 4 - a - e = g*


// be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
// fdgacbe cefdb cefbgd gcbe

use std::collections::{HashSet, HashMap};

struct Segments {
    l: HashMap<char, char>
    // a: Option<char>,
    // b: Option<char>,
    // c: Option<char>,
    // d: Option<char>,
    // e: Option<char>,
    // f: Option<char>,
    // g: Option<char>
}

impl Segments {
    fn new() -> Self {
        return Segments{l: HashMap::new()}
    }

    fn find_a(&self, d: &mut Digits) {
        // 1 = 2 segments
        // 7 = 3 segments
        // 1 - 7 = a
        let one: &HashSet<String> = d.get(&1).unwrap();
        let seven: &HashSet<String> = d.get(&7).unwrap();
        let a = *seven.difference(one).next().unwrap();
        let c: char = a.chars().next().unwrap();
        self.l.insert('a', c);
    }

    fn find_d(self, d: &mut Digits) {
        // 3 - 7 = d, g -> (d, g) and 4 = d*
        let three = d.get(&3).unwrap();
        let seven = d.get(&7).unwrap();
        let dg = three - seven;
        let d = *dg.intersection(d.get(&4).unwrap()).next().unwrap();
        let c: char = d.chars().next().unwrap();
        self.l.insert('d', c);
    }

    fn find_b(self, d: &mut Digits) {
        // 4 - 1 - d = b*
        let four = d.get(&4).unwrap();
        let one = d.get(&1).unwrap();
        for c in four - one {
            if c != self.d.unwrap().to_string() {
                self.b = Some(c.to_char());
            }
        }
    }

    fn find_e(self, d: &mut Digits) {
        let two = d.get(&2).unwrap();
        let three = d.get(&3).unwrap();
        // 2 - 3 = e*
        let e = two - three;
        self.e = Some(e.iter().next().unwrap().chars());
    }

    fn find_f(self, d: &mut Digits) {
        // 2 - 3 = e*, remaining segment in 3 = f*
    }
}

type Digits = HashMap<u16, HashSet<String>>;

fn find_digits(digits: &mut Digits, line: Vec<HashSet<String>>) {
    for d in line {
        if d.len() == 2 {
            digits.insert(1, d);
        }
        else if d.len() == 3 {
            digits.insert(7, d);
        }
        else if d.len() == 4 {
            digits.insert(4, d);
        }
        else if d.len() == 7 {
            digits.insert(8, d);
        }
    }
}

fn find_three(digits: &mut Digits, f: &HashSet<String>) -> bool {
    // if all 1 segment in 5 segment number = 3
    let one: HashSet<String> = *digits.get(&1).unwrap();
    // this line doesn't work if type is HashSet<String>, apparently can't collect on &String
    let five: HashSet<_> = one.difference(&f).collect();
    if five.is_empty() {
        // if the difference is empty, then it's a 5
        digits.insert(3, f.clone());
        return true;
    }
    else {
        return false;
    }
}

fn find_five(digits: &mut Digits, f: &HashSet<String>, s: Segments) -> bool {
    // if 5 segment and b => 5
    let b = s.b.unwrap();
    if f.contains(&b) {
        digits.insert(5, f.clone());
        return true;
    }
    else {
        return false;
    }
}

fn five_segments(digits: & mut Digits, line: Vec<HashSet<String>>, s: Segments) {
    let fs = line.iter().filter(|x| x.len() == 5);
    for f in fs {
        if !find_three(digits, f) && !find_five(digits, f, s) {
            // the remaining five segment is the 2
            digits.insert(2, f.clone());
        }
    }
}


fn parse_line() -> Vec<HashSet<String>> {
    todo!("get parsed vector of the 10 digits")
}

fn main() {
    // parse line
    // map unique digits: 1, 7, 4, 8
}
