use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // we need a different data format and I'm too stripped for time to rewrite the old code, so hackery it is!
    let mut matrix: Vec<String> = Vec::new();
    for line in read_lines(Path::new(filename)) {
        matrix.push(line.unwrap());
    }

    let mut o2plus = matrix.clone();
    let mut bit = 0;
    while o2plus.len() > 1 {
        o2plus = do_search_round(&mut o2plus, 0, bit);
        bit += 1;
    }

    let mut co2minus = matrix.clone();
    bit = 0;
    while co2minus.len() > 1 {
        co2minus = do_search_round(&mut co2minus, 1, bit);
        bit += 1;
    }

    println!("{:}", o2plus[0]);
    println!("{:}", co2minus[0]);
    
    let o2plus_num = isize::from_str_radix(&o2plus[0], 2).unwrap();
    let co2minus_num = isize::from_str_radix(&co2minus[0], 2).unwrap();
    println!("Life Support Rating: {:}", co2minus_num * o2plus_num);
}

fn do_search_round(mx: &mut Vec<String>, which_pattern: usize, bit: usize) -> Vec<String> {
    let mut next = Vec::new();
    let tuple = gamma_epsilon(mx);
    let pattern = if which_pattern == 0 { tuple.0 } else { tuple.1 };
    for candidate in mx.iter() {
        if candidate_matches(&candidate, &pattern, bit) {
            next.push(candidate.clone());
        }
    }
    next
}

fn candidate_matches(cand: &String, pattern: &String, bit: usize) -> bool {
    cand.chars().nth(bit) == pattern.chars().nth(bit)
}

fn gamma_epsilon(mx: &mut Vec<String>) -> (String, String) {
    let mut line_cnt = 0;

    let mut one_counts: Vec<i32> = Vec::new();
    for line in mx.iter() {
        count_ones(&mut one_counts, line);
        line_cnt += 1;
    }
    
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for one_count in one_counts.iter() {
        if one_count == &(line_cnt / 2) {
            gamma.push('1');
            epsilon.push('0');
            continue;
        }
        let next = match one_count > &(line_cnt / 2) {
            true => '1',
            false => '0',
        };
        gamma.push(next);

        let next = match one_count > &(line_cnt / 2) {
            true => '0',
            false => '1',
        };
        epsilon.push(next);
    }
    (gamma, epsilon)
}


fn count_ones(one_counts: &mut Vec<i32>, line: &String) {
    if one_counts.len() < line.len() {
        for _ in 0..line.len() {
            one_counts.push(0);
        }
    }
    for (i, c) in line.chars().enumerate() {
        let val : i8 = c.to_string().parse().unwrap();
        if val > 0 {
            one_counts[i] += val as i32;
        }
    }
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
