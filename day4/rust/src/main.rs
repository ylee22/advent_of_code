use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

// struct BoardInfo {
    // board: Vec<Vec<i32>>,
    //board: HashMap<i32, (i8, i8)>,
    //board_theotherway: HashMap<(i8, i8), i32>,
    
    //board: Vec<i32>
    // row_matches: Vec<usize>,
    // column_matches: Vec<usize>
// }

// impl BoardInfo {
    // fn get(&self, row: usize, col: usize) -> i32 {
    //     //self.board[row * 5 + col]
    //     if(row1 >> 8 & 0xFF)
    // }

    // fn all_nums(&self) -> &[i32] {

    // }

//     fn get_loc(&self, n: i32) -> Option<(usize, usize)> {
//         for idx in 0 .. self.board.len() {
//             if self.board[idx] == n {
//                 return Some((idx / 5, idx % 5));
//             }
//         }

//         return None;
//     }
// }

// fn check_row(bi: &BoardInfo, row: usize, nums: &[i32]) -> Option<usize> {
//     let row = &bi.board[row];
//     let mut hits = 0;

//     for n in nums {
//         todo!()
//     }

//     todo!()
// }

// fn bingo_match(bi: &BoardInfo, num: i32) -> BoardInfo {
//     let mut board: &Vec<Vec<i32>> = &bi.board;
//     let mut row_matches: &Vec<usize> = &bi.row_matches;
//     let mut column_matches: &Vec<usize> = &mut bi.column_matches;
//     let mut row = 0;
//     for line in &bi.board {
//         let column = line.iter().position(|&x| x==num);
//         match column {
//             Some(c) => {bi.column_matches.push(c);
//                               bi.row_matches.push(row)
//                             }
//             _ => continue,
//         }
//         row = row + 1;
//     }

//     bi.row_matches = row_matches;
//     bi.column_matches = column_matches;
    
//     return BoardInfo { board: , row_matches: (), column_matches: () };
// }

fn parse_numbers(line: &String) -> Vec<i32> {
    let split_line = line.split(",").map(|s| s.parse::<i32>().unwrap());
    let nums = split_line.collect();
    return nums;
}

fn parse_board_line(line: &String, b: &mut HashMap<i32,(i32,i32,bool)>, row: i32) {
    let nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let mut col = 0;
    for num in nums {
        // todo!("duplicate numbers in the board")
        b.insert(num, (row, col, false));
        col = col + 1;
    }
}

fn update_board(b: &mut HashMap<i32,(i32,i32,bool)>, num_drawn: i32) {
    if b.contains_key(&num_drawn) {
        let (_,_,f) = b.get_mut(&num_drawn).unwrap();
        *f = true
        // why does the following not work?
        // let f = true;
        // f = true
        // let *f = true;
    } 
}

fn test(b: &HashMap<i32,(i32,i32,bool)>) -> (HashMap<i32,i32>, HashMap<i32,i32>) {
    let mut rm = HashMap::from([(0,0),(1,0),(2,0),(3,0),(4,0)]);
    let mut cm = HashMap::from([(0,0),(1,0),(2,0),(3,0),(4,0)]);
    for (r, c, m) in b.values() {
        if m == &true {
            let ri = rm.get_mut(r).unwrap();
            *ri = *ri + 1;
            let ci = cm.get_mut(c).unwrap();
            *ci = *ci + 1;
        }
    }
    return (rm,cm);
}

fn bingo(b: &HashMap<i32,(i32,i32,bool)>) -> bool {
    let mut rm = HashMap::from([(0,0),(1,0),(2,0),(3,0),(4,0)]);
    let mut cm = HashMap::from([(0,0),(1,0),(2,0),(3,0),(4,0)]);
    for (r, c, m) in b.values() {
        if m == &true {
            let ri = rm.get_mut(r).unwrap();
            *ri = *ri + 1; // THIS DOESN'T UPDATE PAST 1!!! WHY!!!!
            let ci = cm.get_mut(c).unwrap();
            *ci = *ci + 1; // THIS DOESN'T UPDATE PAST 1!!! WHY!!!!
        }
    }
    for v in rm.values() {
        if v == &5 {
            return true
        }
    }
    for v in cm.values() {
        if v == &5 {
            return true
        }
    }
    return false;
}

fn score(b: &HashMap<i32,(i32,i32,bool)>) -> i32 {
    let mut s = 0;
    for (n, (_,_,m)) in b {
        if m == &true {
            s = s + n;
        }
    }
    return s;
}

fn main() {
    let stdin = io::stdin();

    let mut iter_lines = stdin.lock().lines().map(|l| l.unwrap());

    let numbers = parse_numbers(&iter_lines.next().unwrap());

    // TODO: use Vec<HashMap> instead of HashMap<Hashmap>
    // TODO: use struct to name the inner HashMap as board (abstraction)
    // - use impl on a struct to write methods
    // board cell: (i32,i32,bool)
    // mark board or something as the method name for updating the bool
    // TODO: make a board and cell abstraction
    let mut boards: HashMap<i32, HashMap<i32,(i32,i32,bool)>> = HashMap::new();
    let mut board_counter = 0;

    // assemble board
    for line in iter_lines {
        // TODO: blank line
        let mut board = HashMap::new();
        // TODO: detect board size (check the length of the first line)
        for row in 0..5 {
            parse_board_line(&line, &mut board, row)
        }
        boards.insert(board_counter, board);
        board_counter = board_counter + 1;
    }

    // // get bingo
    // for n in numbers {
    //     for b in boards.values_mut() {
    //         update_board(b, n);
    //         let t = bingo(b);
    //         if t {
    //             let s = score(b);
    //             // and then return the score somehow
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_numbers_correct() {
        let result = crate::parse_numbers(&"7,4,9,5,11,17,23,2,0".to_string());
        assert_eq!(result, vec![7,4,9,5,11,17,23,2,0]);
    }

    #[test]
    fn parse_board_line_correct() {
        let mut b = std::collections::HashMap::<i32, (i32,i32,bool)>::new();
        crate::parse_board_line(&"22 13 17 11  0".to_string(), &mut b, 0);
        let f = std::collections::HashMap::from([(22,(0,0,false)),(13,(0,1,false)),(17,(0,2,false)),(11,(0,3,false)),(0,(0,4,false))]);
        assert_eq!(b, f);
    }

    #[test]
    fn update_board_correct() {
        let mut b = std::collections::HashMap::<i32,(i32,i32,bool)>::from([(13,(0,2,false))]);
        crate::update_board(&mut b, 13);
        let (_,_,x) = b.get(&13).unwrap();
        assert_eq!(x,&true);
    }

    #[test]
    fn bingo_row() {
        let b = std::collections::HashMap::<i32,(i32,i32,bool)>::from([
            (0,(0,0,true)),
            (1,(1,0,true)),
            (2,(2,0,true)),
            (3,(3,0,true)),
            (4,(4,0,true))]);
        println!("{:?}", b);
        let (_,cm) = crate::test(&b);
        let c = cm.get(&0);
        assert_eq!(c.unwrap(),&5);
    }

}