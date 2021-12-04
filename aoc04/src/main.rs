use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let (draws, mut boards) = read_input(filename);

    let mut bingo = false;
    for num in draws {
        let mut winning_boards = Vec::<usize>::new();
        for (i, board) in boards.iter_mut().enumerate() {
            board.strikeout_all(&num);
            if board.has_full_row() || board.has_full_column() {
                println!("Bingo! board {} is winner", i);
                println!("Last called number: {}    Winning score: {}", num, board.score(num));
                
                winning_boards.push(i);
            }
        }
        winning_boards.reverse();
        for winner in winning_boards.iter() {
            boards.remove(*winner);
        }
    }

    for board in boards.into_iter() {
        board.print();
        println!("");
    }
}

struct Board {
    rows: Vec<Vec<i32>>
}

impl Board {

    fn strikeout_all(&mut self, num: &i32) {
        for y in 0..5 {
            for x in 0..5 {
                if &self.rows[y][x] == num {
                    self.rows[y][x] = -num
                }
            }
        }
    }

    fn has_full_row(&self) -> bool {
        self.rows.iter().any(|row| row.iter().all(|num| num < &0 ))
    }
    fn has_full_column(&self) -> bool {
        (0..5).any(|colidx| self.rows.iter().all(|row| &row[colidx] < &(colidx as i32)))
    }

    fn fmt_cell(cell : &i32) -> String {
        if cell < &0 {
            format!("[{num:>width$}]", num=cell.abs(), width=2)
        } else {
            format!("{num:>width$} ", num=cell, width=3)
        }
    }

    fn print(&self) {
        for row in &self.rows {
            println!("{:} {:} {:} {:} {:}", Board::fmt_cell(&row[0]), Board::fmt_cell(&row[1]), Board::fmt_cell(&row[2]), Board::fmt_cell(&row[3]), Board::fmt_cell(&row[4]))
        }
    }

    fn score(&self, last_call: i32) -> i32 {
        last_call * self.rows.iter().fold(0, |acc, row| acc + row.iter().map(|x| x.max(&0)).sum::<i32>() )
    }
}

fn read_input(filename: &String) -> (Vec<i32>, Vec<Board>) {
    let mut iter = read_lines(filename);
    let mut lots : Vec<i32> = Vec::new();
    for draw in iter.next().unwrap().unwrap().split(',') {
        lots.push(draw.parse::<i32>().unwrap());
    }

    let mut boards = Vec::<Board>::new();
    while let Some(Ok(mut line)) = iter.next() {
        let mut rows : Vec<Vec<i32>> = Vec::new();
        for _ in 0..5 {
            line = iter.next().unwrap().unwrap();
            let row : Vec<i32> = line.split_whitespace().map(|literal| literal.parse::<i32>().unwrap()).collect();
            rows.push(row)
        }
        boards.push(Board { rows: rows });

    }

    (lots, boards)
}


fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
