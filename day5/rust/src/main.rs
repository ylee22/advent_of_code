use std::{io::{self, BufRead}, collections::{HashMap}, cmp};

#[derive(Clone,Copy,Eq,Hash,Debug)]
struct Coord {
    x: u32,
    y: u32
}

#[derive(Debug,PartialEq)]
struct Line {
    start: Coord,
    end: Coord,
}

impl Iterator for Line {
    type Item=Coord;

    fn next(&mut self) -> Option<Coord> {
        let tx = self.start.x;
        let ty = self.start.y;

        if self.start.x > self.end.x && self.start.y > self.end.y {
            return None
        }

        if self.start.x <= self.end.x {
            self.start.x += 1;
        }

        if self.start.y <= self.end.y {
            self.start.y += 1;
        }

        return Some(Coord::new(cmp::min(tx, self.end.x), cmp::min(ty, self.end.y)))         

    }
}

impl Coord {
    fn new(x: u32, y: u32) -> Self {
        return Coord{x, y}
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        if (self.x == other.x) && (self.y == other.y) {
            return true
        }
        else {
            return false
        }
    }
}

fn intersect(lines: Vec<Line>) -> u32 {
    // (1, 2, 3)
    // 1 -> in (1,2,3)
    // 1 -> in (1,2,3)
    let mut seen: HashMap<Coord, u32> = HashMap::new();
    for mut line in lines {
        loop {
            match line.next() {
                Some(coord) =>
                    if seen.contains_key(&coord) {
                        *seen.get_mut(&coord).unwrap() += 1
                    } else {
                        seen.insert(coord, 1);
                    },
                None => break,
            }
        }
    }

    let mut multiple: u32 = 0;
    for v in seen.values() {
        if v > &1 {
            multiple += 1;
        }
    }
    return multiple
}

fn parse_int(s: &str) -> u32 {
    let parsed = s.trim().parse::<u32>();
    match parsed {
        Ok(u) => u,
        Err(_) => panic!("Could not parse {} as u32", s)
    }
}

fn parse_lines<T: Iterator<Item=String>>(lines: T) -> Vec<Line>{
    let mut vents: Vec<Line> = Vec::new();

    for line in lines {
        let mut str_parts = line.split("->");
        let mut coord1 = str_parts.next().unwrap().split(",").map(|s| parse_int(s));
        let x1 = coord1.next().unwrap();
        let y1 = coord1.next().unwrap();
        let mut coord2 = str_parts.next().unwrap().split(",").map(|s| parse_int(s));
        let x2 = coord2.next().unwrap();
        let y2 = coord2.next().unwrap();

        let l = 
            if (x1 < x2) | (y1 < y2) {
                Line{start: Coord::new(x1,y1), end: Coord::new(x2,y2)}
            }
            else {
                Line{start: Coord::new(x2,y2), end: Coord::new(x1,y1)}
            };

        match diagonal_line(&l) {
            false => vents.push(l),
            true => (),
        }

        // vents.push(l);

    }

    return vents
}

fn diagonal_line(l: &Line) -> bool {
    if (l.start.x == l.end.x) | (l.start.y == l.end.y) {
        return false
    }
    else {
        return true
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());
    let vents = parse_lines(lines);
    let multiple = intersect(vents);
    println!("{}", multiple)
}

#[cfg(test)]
mod tests {
    use crate::{Coord, Line, intersect, parse_lines, diagonal_line};

    #[test]
    fn test_next_diag() {
        let mut line = Line{start: Coord::new(0,0), end: Coord::new(2,2)};
        let c1 = line.next();
        assert_eq!(c1, Some(Coord::new(0,0)));
        let c2 = line.next();
        assert_eq!(c2, Some(Coord::new(1,1)));
        let c3 = line.next();
        assert_eq!(c3, Some(Coord::new(2,2)));
        let c4 = line.next();
        assert_eq!(c4, None);        
    }

    #[test]
    fn test_next_x() {
        let mut line = Line{start: Coord::new(1,5), end: Coord::new(3,5)};
        let c1 = line.next();
        assert_eq!(c1, Some(Coord::new(1,5)));
        let c2 = line.next();
        assert_eq!(c2, Some(Coord::new(2,5)));
        let c3 = line.next();
        assert_eq!(c3, Some(Coord::new(3,5)));
        let c4 = line.next();
        assert_eq!(c4, None);        
    }

    #[test]
    fn test_next_y() {
        let mut line = Line{start: Coord::new(0,3), end: Coord::new(0,5)};
        let c1 = line.next();
        assert_eq!(c1, Some(Coord::new(0,3)));
        let c2 = line.next();
        assert_eq!(c2, Some(Coord::new(0,4)));
        let c3 = line.next();
        assert_eq!(c3, Some(Coord::new(0,5)));
        let c4 = line.next();
        assert_eq!(c4, None);        
    }

    #[test]
    fn test_intersect_diag() {
        let line1 = Line{start: Coord::new(0,0), end: Coord::new(2,2)};
        let line2 = Line{start: Coord::new(1,1), end: Coord::new(5,5)};
        let v = vec![line1, line2];
        let count = intersect(v);
        assert_eq!(count, 2)
    }

    #[test]
    fn test_parse() {
        let lines = vec!["0,9 -> 5,9".to_string(), "8,0 -> 0,8".to_string()];
        let vents = parse_lines(lines.into_iter());
        assert_eq!(vents[0].start, Coord::new(0,9));
        assert_eq!(vents[0].end, Coord::new(5,9));
        assert_eq!(vents[1].start, Coord::new(8,0));
        assert_eq!(vents[1].end, Coord::new(0,8));
    }

    #[test]
    fn test_filter_vertical() {
        let line = Line{start: Coord::new(0, 0), end: Coord::new(0, 5)};
        let b = diagonal_line(&line);
        assert_eq!(b, false);
    }

    #[test]
    fn test_filter_horizontal() {
        let line = Line{start: Coord::new(5, 0), end: Coord::new(0, 0)};
        let b = diagonal_line(&line);
        assert_eq!(b, false);
    }
}