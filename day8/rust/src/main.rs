// unique:
// 2 segments: 1 (c, f)
// 3 segments: 7 (a, c, f)
// 4 segments: 4 (b, c, d, f)
// 7 segments: 8

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

use std::{collections::{HashSet, HashMap}, io::{self, BufRead}};

#[derive(Clone)]
struct CharMap {
    map: HashMap<char, char>
}

impl CharMap {
    fn new() -> Self {
        return CharMap{map: HashMap::new()}
    }

    fn find_a(&mut self, d: &mut Digits) {
        // 1 = 2 segments
        // 7 = 3 segments
        // 7 - 1 = a
        let one: &HashSet<char> = d.map.get(&1).unwrap();
        let seven: &HashSet<char> = d.map.get(&7).unwrap();
        let a = seven.difference(one).next().unwrap().clone();
        self.map.insert('a', a);
    }

    fn find_d_g(&mut self, d: &mut Digits) {
        // 3 - 7 = d, g -> (d, g) and 4 = d
        let three = d.map.get(&3).unwrap();
        let mut seven: HashSet<char> = d.map.get(&7).unwrap().clone();
        let dg = three - &seven;
        let d = dg.intersection(d.map.get(&4).unwrap()).next().unwrap().clone();
        self.map.insert('d', d);
        // 3 - 7 - d = g
        seven.insert(d);
        let g = three - &seven;
        let g: char = *g.iter().next().unwrap();
        self.map.insert('g', g);
    }

    fn find_b(mut self, d: &mut Digits) {
        // 4 - 1 - d = b
        let four = d.map.get(&4).unwrap();
        let mut one = d.map.get(&1).unwrap().clone();
        // add d to one, then it should be equivalent to 4 - 1 - d
        one.insert(*self.map.get(&'d').unwrap());
        let c = four - &one;
        let b = c.iter().next().unwrap();
        self.map.insert('b', *b);
    }

    fn find_e_f(mut self, d: &mut Digits) {
        let two = d.map.get(&2).unwrap();
        let three = d.map.get(&3).unwrap();
        let e = two - three;
        // 2 - 3 = e
        let e = e.iter().next().unwrap();
        // 3 - 2 = f
        let f = three - two;
        let f = f.iter().next().unwrap();
        self.map.insert('e', *e);
        self.map.insert('f', *f);
    }

    fn find_c(mut self, d: &mut Digits) {
        // 1 - f = c
        let one = d.map.get(&1).unwrap();
        let mut f: HashSet<char> = HashSet::new();
        f.insert(*self.map.get(&'f').unwrap());
        let c = one - &f;
        let c: char = *c.iter().next().unwrap();
        self.map.insert('c', c);
    }

}

struct Digits {
    map: HashMap<u16, HashSet<char>>
}

impl Digits {
    fn new() -> Self {
        return Digits{map: HashMap::new()}
    }

    fn find_three(&mut self, f: &HashSet<char>) -> bool {
        // if all segments from 1 in 5 segment number = 3
        let one: &HashSet<char> = self.map.get(&1).unwrap();
        let five: HashSet<_> = one.difference(&f).collect();
        if five.is_empty() {
            // if the difference is empty, then it's a 5
            self.map.insert(3, f.clone());
            return true;
        }
        else {
            return false;
        }
    }

    fn find_five(&mut self, f: &HashSet<char>, c: &CharMap) -> bool {
        // if 5 segment and b => 5
        let b: &char = c.map.get(&'b').unwrap();
        if f.contains(b) {
            self.map.insert(5, f.clone());
            return true;
        }
        else {
            return false;
        }
    }

    fn find_two_three_five(&mut self, five_segs: Vec<HashSet<char>>, cmap: &CharMap) {
        // find 3, 5, 2 from the 5 segments and the chars
        let mut three_found = false;
        let mut five_found = false;
        for f in five_segs {
            // there are 3 5 segment digits: 2, 3, 5
            three_found = self.find_three(&f);
            five_found = self.find_five(&f, &cmap);
            if three_found & five_found {
                // remaining 5 segment digit is 2
                self.map.insert(2, f.clone());
            }
            else if !three_found & !five_found {
                self.map.insert(2, f.clone());
            }
        }
    }

    fn find_zero(&mut self, s: &HashSet<char>, cmap: &CharMap) {
        // six is 6 segment digit without a c
        let c = cmap.map.get(&'d').unwrap();
        if s.contains(c) {
            self.map.insert(0, s.clone());
        }
    }

    fn find_six(&mut self, s: &HashSet<char>, cmap: &CharMap) {
        // six is 6 segment digit without a c
        let c = cmap.map.get(&'c').unwrap();
        if s.contains(c) {
            self.map.insert(6, s.clone());
        }
    }

    fn find_nine(&mut self, s: &HashSet<char>, cmap: &CharMap) {
        // six is 6 segment digit without a c
        let c = cmap.map.get(&'e').unwrap();
        if s.contains(c) {
            self.map.insert(9, s.clone());
        }
    }

    fn find_zero_six_nine(&mut self, six_segs: Vec<HashSet<char>>, cmap: &CharMap) {
        for s in six_segs {
            self.find_zero(&s, cmap);
            self.find_six(&s, cmap);
            self.find_nine(&s, cmap);
        }
    }
}

fn find_unique_digits(digits: &mut Digits, line: Vec<HashSet<char>>) -> (Vec<HashSet<char>>, Vec<HashSet<char>>) {
    // store the 5 segments
    let mut five: Vec<HashSet<char>> = Vec::new();
    let mut six: Vec<HashSet<char>> = Vec::new();

    for d in line {
        if d.len() == 2 {
            digits.map.insert(1, d);
        }
        else if d.len() == 3 {
            digits.map.insert(7, d);
        }
        else if d.len() == 4 {
            digits.map.insert(4, d);
        }
        else if d.len() == 7 {
            digits.map.insert(8, d);
        }
        else if d.len() == 5 {
            five.push(d);
        }
        else if d.len() == 6 {
            six.push(d);
        }
    }

    return (five, six)
}

fn get_hashset(v: Vec<String>) -> Vec<HashSet<char>> {
    let mut l: Vec<HashSet<char>> = Vec::new();
    for s in v {
        l.push(HashSet::from_iter(s.chars()));
    }
    return l
}

fn parse_line(line: &String) -> (Vec<String>, Vec<String>) {
    // be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
    // fdgacbe cefdb cefbgd gcbe
    let mut line = line.split('|');
    let digits: Vec<String> = line.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();
    let output: Vec<String> = line.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();
    return (digits, output)
}

fn main() {
    let stdin = io::stdin();
    let mut iter_lines = stdin.lock().lines().map(|l| l.unwrap());
    let (d_line, output) = parse_line(&iter_lines.next().unwrap());
    // these are the scrambled digits
    let d_line = get_hashset(d_line);
    // this is the scrambled time, which will get descrambled
    let output = get_hashset(output);

    let mut digits: Digits = Digits::new();    
    // first pass over the line
    let (five_segs, six_segs) = find_unique_digits(&mut digits, d_line);
    
    // find chars
    let mut cmap = CharMap::new();
    cmap.find_a(&mut digits);
    cmap.find_b(&mut digits);

    // find 3, 5, 2 from the 5 segments and the chars
    digits.find_two_three_five(five_segs, &cmap);

    // need to find the 3 digit first
    cmap.find_d_g(&mut digits);

    // find e and f
    cmap.find_e_f(&mut digits);

    // find c
    cmap.find_c(&mut digits);

    // at this point we have all the chars, so fill in the rest of the digits map
    digits.find_zero_six_nine(six_segs, &cmap);

    // now identify the output
    let mut decoded = Vec::<u16>::new();
    for d in output {
        // find the matching set in the digit
        for (k, v) in &digits.map {
            if v==&d {
                decoded.push(k.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashSet, HashMap};
    use crate::{parse_line, get_hashset, find_unique_digits, Digits};

    #[test]
    fn test_parse_line() {
        let s = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string();
        let (d, o) = parse_line(&s);
        println!("{:?}", d);
        println!("{:?}", o);
        assert_eq!(vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"], o)
    }

    #[test]
    fn test_get_hashset() {
        let s = vec!["be".to_string(), "cfbegad".to_string()];
        let v = get_hashset(s);
        assert_eq!(vec![HashSet::from(['b', 'e']), HashSet::from(['c','f','b','e','g','a','d'])], v);
    }

    #[test]
    fn test_find_unique_digits() {
        let mut digits: Digits = Digits::new();
        let d_line = vec![HashSet::from(['b', 'e']), HashSet::from(['c','f','b','e','g','a','d']),
                                              HashSet::from(['c','g','e','b']), HashSet::from(['e','d','b']),
                                              HashSet::from(['f','d','c','g','e']), HashSet::from(['f','e','c','d','b'])];
        // len 2 = 1, len 3 = 7, len 4 = 4, len 7 = 8
        let (five, _) = find_unique_digits(&mut digits, d_line);
        println!("{:?}", five);
        assert_eq!(five, vec![HashSet::from(['f','d','c','g','e']), HashSet::from(['f','e','c','d','b'])]);
        println!("{:?}", digits.map);
        assert_eq!(digits.map.get(&1).unwrap().clone(), HashSet::from(['b', 'e']));
        assert_eq!(digits.map.get(&8).unwrap().clone(), HashSet::from(['c','f','b','e','g','a','d']));
        assert_eq!(digits.map.get(&4).unwrap().clone(), HashSet::from(['c','g','e','b']));
        assert_eq!(digits.map.get(&7).unwrap().clone(), HashSet::from(['e','d','b']));
    }


}