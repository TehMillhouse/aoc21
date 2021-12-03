use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut line_cnt = 0;
    
    let mut one_counts: Vec<i32> = Vec::new();
    for line in read_lines(Path::new(filename)) {
        count_ones(&mut one_counts, line.unwrap());
        line_cnt += 1;
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for (i, one_count) in one_counts.iter().enumerate() {
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
    println!("{}", gamma);
    println!("{}", epsilon);

    let gamma_num = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_num = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("{:}", gamma_num);
    println!("{:}", epsilon_num);
    println!("Power consumption: {:}", gamma_num * epsilon_num);
    
}

fn count_ones(one_counts: &mut Vec<i32>, line: String) {
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
