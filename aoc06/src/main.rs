use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::string::ParseError;
use std::str::FromStr;
use std::env;
use regex::Regex;
use std::cmp;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let fishies = read_input(filename);
    let mut timers = [0_u64; 9];
    for fish in fishies.iter() {
        timers[*fish as usize] += 1;
    }
    
    let limit = 256;
    for tick in 0..limit {
        let spawn_idx = wrap(tick);
        let inc = &timers[spawn_idx as usize];
        timers[wrap(tick - 2) as usize] += *inc;
        //pprint(timers, spawn_idx);
    }
    println!("{}", timers.iter().sum::<u64>());
}

fn pprint(timers: [u64; 9], spawn_idx: i32) {
    let mut s = String::new();
    for i in 0..9 {
        s.push_str(&format!("{}, ", timers[wrap(i + spawn_idx) as usize]))
    }
    println!("{}", s);
}

fn wrap(i: i32) -> i32 {
    i.rem_euclid(9)
}

fn read_input(filename: &String) -> Vec<i32> {
    let mut iter = read_lines(filename);
    let line = iter.next().unwrap().unwrap();
    let split = line.split(",");
    split.map(|x| x.parse::<i32>().unwrap() ).collect::<Vec<i32>>()
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
