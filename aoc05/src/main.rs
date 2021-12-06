use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::string::ParseError;
use std::str::FromStr;
use std::env;
use regex::Regex;
use std::cmp;

struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+),(\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        Ok(Point {
            x: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(), 
            y: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(), 
        })
    }
}

struct Board {
    cells: [[u8; 1000]; 1000],
}

fn lerp(start: i32, end: i32, t: f32) -> f32 {
    let startf = start as f32;
    let endf = end as f32;
    startf + t * (endf - startf)
}

impl Board {
    fn draw(self: &mut Self, from: &Point, to: &Point) {
        let xdist = (to.x - from.x).abs();
        let ydist = (to.y - from.y).abs();
        let cell_num = cmp::max(xdist, ydist);
        for i in 0..cell_num {
            let t = (i as f32) / (cell_num as f32); // lerp value
            let x = lerp(from.x, to.x, t).round() as usize;
            let y = lerp(from.y, to.y, t).round() as usize;
            self.cells[y][x] += 1;
        }
        self.cells[to.y as usize][to.x as usize] += 1;
    }

    fn print(self: &mut Self, size: i32) {
        for row in 0..size {
            let mut s = String::new();
            for col in 0..size {
                let cell = self.cells[row as usize][col as usize];
                s.push_str(format!("{:}", cell).as_str());
            }
            println!("{:}", s);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut board = Board{ cells: [[0u8; 1000]; 1000]};
    let vents = read_input(filename);

    for (from, to) in vents.iter() {
        //if from.x != to.x && from.y != to.y {
        //    continue;
        //}
        board.draw(from, to);
        board.print(10);
        println!("================");
    }

    let mut supervent = 0;
    for row in board.cells.iter() {
        for cell in row {
            if *cell > 1 {
                supervent += 1;
            }
        }
    }
    println!("{}", supervent);
}

fn read_input(filename: &String) -> Vec<(Point, Point)> {
    let iter = read_lines(filename);
    let mut vents : Vec<(Point, Point)> = Vec::new();
    for line in iter {
        let line2 = line.unwrap();
        let split = line2.split(" -> ");
        let points = split.collect::<Vec<&str>>();
        vents.push( (points[0].parse().unwrap(), points[1].parse().unwrap()) );
        
    }
    vents
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
