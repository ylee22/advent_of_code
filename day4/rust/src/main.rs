use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug,PartialEq)]
struct Board{
    cells: HashMap<Value, Cell>,
    won: i32,
}

type Value = i32;

#[derive(Debug,PartialEq)]
struct Cell {
    row: i32,
    col: i32,
    mark: bool,
}

impl Cell {
    fn new(row: i32, col: i32) -> Self {
        return Cell{row, col, mark:false}
    }
}

impl Board {
    fn new(v: Vec<(i32,i32,i32)>) -> Self {
        let spaces = v.iter().map(|(space, r, c)| (*space, Cell::new(*r, *c)));
        let hash : HashMap<i32, Cell> = HashMap::from_iter(spaces);

        // let board = HashMap::new();
        // for (space, r, c) in v {
        //     board.insert(space, Cell::new(r,c));
        // }
        
        return Board{cells: hash, won: 0}
    }

    fn get_loc(&self, number: i32) -> Option<(i32, i32)> {
        let cell = self.cells.get(&number);
        match cell {
            Some(c) => { Some((c.row, c.col)) }
            None => { None }
        }
    }

    fn update(&mut self, n: i32) {
        if self.cells.contains_key(&n) {
            self.cells.get_mut(&n).unwrap().mark = true;
        }
    }

    fn bingo(&self) -> bool {
        let mut rs = HashMap::from([(0,0),(1,0),(2,0),(3,0),(4,0)]);
        let mut cs = HashMap::from([(0,0),(1,0),(2,0),(3,0),(4,0)]);
        for cell in self.cells.values() {
            if cell.mark == true {
                let r = rs.get_mut(&cell.row).unwrap();
                *r = *r + 1;
                let c = cs.get_mut(&cell.col).unwrap();
                *c = *c + 1;
            }
        }
        for count in rs.values() {
            if count == &5 {
                return true
            }
        }
        for count in cs.values() {
            if count == &5 {
                return true
            }
        }
        return false
    }

    fn score (&self, n: i32) -> i32 {
        let mut s = 0;
        for (num, cell) in self.cells.iter() {
            if cell.mark == false {
                s = s + num
            }
        }
        return s * n
    }
}

fn parse_numbers(line: &String) -> Vec<i32> {
    let split_line = line.split(",").map(|s| s.parse::<i32>().unwrap());
    let nums = split_line.collect();
    return nums;
}

fn parse_board_line(line: &String, b: &mut Board, row: i32) {
    let nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let mut col = 0;
    for num in nums {
        // todo!("duplicate numbers in the board")
        b.cells.insert(num, Cell::new(row, col));
        col = col + 1;
    }
}

fn assemble_boards<T: Iterator<Item=String>>(iter_lines: T) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
   
    let mut lns = iter_lines.filter(|l| !l.is_empty());

    loop {
        let first_line = 
            match lns.next() {
                None => return boards,
                Some(line) => line,
            };
        
        let mut board = Board{ cells: HashMap::new(), won: 0 };
        parse_board_line(&first_line, &mut board, 0);
        for i in 1..5 {
            parse_board_line(&lns.next().unwrap(), &mut board, i);
        }
        boards.push(board);
    }
}

fn main() {
    let stdin = io::stdin();

    let mut iter_lines = stdin.lock().lines().map(|l| l.unwrap());

    let numbers = parse_numbers(&iter_lines.next().unwrap());

    // assemble board
    let mut boards = assemble_boards(iter_lines);

    // get bingo
    let mut won: i32 = 0;
    for n in numbers {
        for (i, b) in boards.iter_mut().enumerate() {
            b.update(n);
            let t = b.bingo();
            if t & (b.won == 0){
                won = won + 1;
                // if won, mark board as won
                b.won = won;
                let s = b.score(n);
                
                println!("{}: {}",i,s);
                // panic!("How do you exit gracefully?")
                // use return with no arguments to exit gracefully
                // return ;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{Cell, Board, assemble_boards};

    #[test]
    fn parse_numbers_correct() {
        let result = crate::parse_numbers(&"7,4,9,5,11,17,23,2,0".to_string());
        assert_eq!(result, vec![7,4,9,5,11,17,23,2,0]);
    }

    #[test]
    fn parse_board_line_correct() {
        let correct_board = crate::Board::new(vec![
            (22,0,0),(13,0,1),(17,0,2),(11,0,3),(0,0,4),
            (22,1,0),(13,1,1),(17,1,2),(11,1,3),(0,1,4),
            ]);

        let mut test_board = crate::Board{cells: HashMap::new(), won: 0};
        crate::parse_board_line(&"22 13 17 11  0".to_string(), &mut test_board, 1);
        assert_eq!(test_board, correct_board);
    }

    #[test]
    fn update_board_correct() {
        let mut b = crate::Board::new(vec![(13,0,2)]);
        b.update(13);
        let x:&Cell = b.cells.get(&13).unwrap();
        assert_eq!(x.mark,true);
    }

    #[test]
    fn bingo_col() {
        let mut b: Board = Board::new(vec![(0,0,0),(1,1,0),(2,2,0),(3,3,0),(4,4,0)]);
        for v in 0..5 {
            b.update(v)
        }
        let x = b.bingo();
        assert_eq!(x, true);
    }

    #[test]
    fn assemble_boards_correct() {
        let lines = vec!["",
            "68 73 98 51 49",
            "82 56 87 64  8",
            "46  7 21 38 30",
            "66  5 86 97 74",
            "60 63 76 55 39",
            "",
            "92 20 87 77 52",
            "72 29 81 24 64",
            "26 16 19 79 68",
            "8 53 90 14 74",
            "28 89 78 54 15",
            "",
            "13 17 35  2 85",
            "37 87 57 74 65",
            "60 21 18 98 96",
            "4 51 46 84  0",
            "90 75 80 41 64",
            ""];
        
        let lines_iter = lines.iter().map(|x| x.to_string());

        let test_boards = assemble_boards(lines_iter);
        
        assert_eq!(test_boards[0].get_loc(7), Some((2,1)));
        assert_eq!(test_boards[1].get_loc(52), Some((0,4)));
        assert_eq!(test_boards[2].get_loc(-100), None);
    }

    #[test]
    fn mark_won() {
        let mut b: Board = Board::new(vec![(0,0,0),(1,1,0),(2,2,0),(3,3,0),(4,4,0)]);
        assert_eq!(0, b.won);
        b.won = 1;
        assert_eq!(1, b.won)
    }
}