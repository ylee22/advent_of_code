use std::io::{self, BufRead};


struct Crabs {
    pos: Vec<u32>
}

impl Crabs {
    fn calc_fuel(&self, pos: u32) -> u32 {
        let mut fuel = 0;
        let first = self.pos.iter().min().unwrap();
        for crab in self.pos.iter() {
            // proof of arithmetic sum:
            // S = a + (a + 1d) + (a + 2d) ... + (a + (n - 1)d)
            // S = (a + (n - 1)d) + (a + (n - 2)d) ... + a
            // 2S = (2a + (n - 1)d) + (2a + (n - 1)d) ... + (2a + (n - 1)d)
            // 2S = n(2a + (n - 1)d)
            // S = (n/2)(2a + (n - 1)d)
            // S = (n/2)(a + (a + (n - 1)d))
            // for consecutive numbers, d = 1
            // S = (n/2)(a + (a + (n - 1)))
            // a + n - 1 = last number in the sequence
            // S = (n/2)(first + last)
            // (a + n - 1) - a + 1 = n
            let last: u32 = crab.abs_diff(pos);
            let f: u32 = (last - first + 1)*(first + last)/2;
            fuel = fuel + f;
        }
        return fuel
    }

    fn find_least_fuel(&self) -> u32 {
        let max_pos = self.pos.iter().max().unwrap();
        let mut fuel = 0;

        for p in 0..*max_pos {
            let curr_fuel = self.calc_fuel(p);
            if p == 0 || curr_fuel < fuel {
                fuel = curr_fuel
            }
        }
        return fuel
    }
}

fn parse_int(s: &str) -> u32 {
    let parsed = s.trim().parse::<u32>();
    match parsed {
        Ok(u) => u,
        Err(_) => panic!("Could not parse {} as u32", s)
    }
}

fn parse_crabs(line: &String) -> Crabs {
    let pos = line.split(",").map(|s| s.parse::<u32>().unwrap()).collect();
    return Crabs { pos: pos }
}

fn main() {
    let stdin = io::stdin();
    let mut iter_lines = stdin.lock().lines().map(|l| l.unwrap());
    let crabs = parse_crabs(&iter_lines.next().unwrap());
    let fuel = crabs.find_least_fuel();
    println!("fuel: {}", fuel)
}

#[cfg(test)]
mod tests {
    use crate::{Crabs, parse_crabs};

    #[test]
    fn test_parse_crabs() {

        let s = "16,1,2,0,4,2,7,1,2,14".to_string();
        let c = parse_crabs(&s);
        assert_eq!(vec![16,1,2,0,4,2,7,1,2,14], c.pos)
    }

    #[test]
    fn test_calc_fuel() {
        let s = "16,1,2,0,4,2,7,1,2,14".to_string();
        let c: Crabs = parse_crabs(&s);
        let p = 5;
        let f = c.calc_fuel(p);
        assert_eq!(f, 168)
    }

    #[test]
    fn test_least_fuel() {
        let s = "16,1,2,0,4,2,7,1,2,14".to_string();
        let c: Crabs = parse_crabs(&s);
        let fuel = c.find_least_fuel();
        assert_eq!(fuel, 168)
    }
}