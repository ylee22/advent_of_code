use std::{io::{self, BufRead}};

type Fishes = [u128; 9];

fn parse_lines(line: String) -> Fishes {
    let fish_iter =  line.split(',');
    let mut fishes = [0; 9];
    for fish in fish_iter {
        let i = fish.trim().parse::<usize>().unwrap();
        fishes[i] = fishes[i] + 1;
    }
    return fishes
}

fn increment_fishes(fishes: &Fishes) -> Fishes {
    let mut updated_fishes = [0; 9];
    // println!("{:?}", fishes);

    for i in (0..fishes.len()).rev() {
        // all fishes are moved from current bucket down to the lower bucket as they age
        if i > 0 {
            updated_fishes[i-1] = fishes[i];
        }
        else {
            // all fishes in the 0 timer bucket are reset to timer 6 bucket
            // new fishes are also added with timer = 8
            updated_fishes[8] = updated_fishes[8] + fishes[i];
            updated_fishes[6] = updated_fishes[6] + fishes[i];
        }
    }

    return updated_fishes
}

fn run_increment_fishes(mut fishes: Fishes, days: u32) -> Fishes{
    for d in 0..days {
        // println!("days: {}", d);
        fishes = increment_fishes(&fishes);
    }
    return fishes
}

fn main() {
    let days: u32 = 256;

    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    let line = line_iter.next().unwrap().unwrap();
    let mut fishes: Fishes = parse_lines(line);

    fishes = run_increment_fishes(fishes, days);

    println!("{} fishes", fishes.iter().sum::<u128>())
    }


#[cfg(test)]
mod tests {
    use crate::{run_increment_fishes};

    #[test]
    fn test_fish_count() {
        let mut fishes = [0; 9];
        fishes[0] = 3;
        println!("{:?}", fishes);
        fishes = run_increment_fishes(fishes, 1);
        assert_eq!(fishes, [0,0,0,0,0,0,3,0,3]);
    }

    #[test]
    fn test_increment_fishes() {
        let day: u32 = 18;
        let mut fishes = [0; 9];
        fishes[1] = 1;
        fishes[2] = 1;
        fishes[3] = 2;
        fishes[4] = 1;
        fishes = run_increment_fishes(fishes, day);
        println!("{:?}", fishes);
        assert_eq!(fishes.iter().sum::<u128>(), 26)

    }
}