use core::fmt;
use std::io::{self, BufRead};

#[derive(Clone,PartialEq)]
struct Fish {
    timer: u16
}

impl Fish {
    fn new(timer: u16) -> Self {
        return Fish{timer} 
    }

    fn count(& mut self) -> Option<Fish> {
        let mut new_fish = None;

        if self.timer == 0 {
            new_fish = Some(Fish::new(8));
            self.timer = 6;
        }
        else {
            self.timer -= 1;
        }

        return new_fish
    }
}

impl fmt::Debug for Fish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.timer)
    }
}

fn parse_lines(line: String) -> Vec<Fish> {
    let fish_iter =  line.split(',');
    let mut fishes = Vec::new();
    for fish in fish_iter {
        fishes.push(Fish::new(fish.trim().parse::<u16>().unwrap()))
    }
    return fishes
}

fn increment_fishes(fishes: &mut Vec<Fish>, days: u16) {
    // why do I need the reference to mut Vec<Fish>?
    // if i make a reference of fishes, then will adding more to more_fishes also add more to fishes?
    let mut new_fishes: Vec<Fish> = Vec::new();

    for fish in fishes.iter_mut() {
        let new_fish = fish.count();
        match new_fish {
            Some(f) => new_fishes.push(f),
            None => ()
        }
    }

    fishes.append(&mut new_fishes);

    if days > 0 {
        increment_fishes(fishes, days - 1)
    }
}

fn main() {
    let days: u16 = 80 - 1;

    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    let line = line_iter.next().unwrap().unwrap();
    let mut fishes: Vec<Fish> = parse_lines(line);

    increment_fishes(&mut fishes, days);

    println!("{} fishes", fishes.len())
    }


#[cfg(test)]
mod tests {
    use crate::{Fish, increment_fishes};

    #[test]
    fn test_fish_count() {
        let mut fish = Fish::new(1);
        println!("{:?}", fish);
        let new_fish = fish.count();
        assert_eq!(new_fish, None);
        assert_eq!(fish.timer, 0);
        let new_fish = fish.count().unwrap();
        println!("{:?}", new_fish);
        println!("{:?}", fish);
        assert_eq!(new_fish.timer, 8);
        assert_eq!(fish.timer, 6)
    }

    #[test]
    fn test_increment_fishes() {
        // let day: u16 = 3;
        // let mut fishes = vec![Fish::new(1, true)];
        // increment_fishes(&mut fishes, day);
        // assert_eq!(fishes.len(), 2);

        let day: u16 = 18;
        let mut fishes = vec![Fish::new(3), Fish::new(4), Fish::new(3), Fish::new(1), Fish::new(2)];
        increment_fishes(&mut fishes, day-1);
        println!("{:#?}", fishes);
        assert_eq!(fishes.len(), 26)

    }
}