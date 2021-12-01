use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    main_a();
    main_b();
}

fn main_a() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut last : i64 = i64::MAX;
    let mut cur : i64;
    let mut increases : i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let line = line.unwrap();
            cur = line.parse::<i64>().unwrap();
            if cur > last {
                increases += 1;
            }
            last = cur;
        }
        println!("{}", increases);
    }
}

fn main_b() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut window: [i32; 3] = [0; 3];
    let mut rotidx : usize = 0;
    let mut cur : i32;
    let mut last : i32 = i32::MAX;
    let mut increases : i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let line = line.unwrap();
            cur = line.parse::<i32>().unwrap();
            window[rotidx] = cur;
            let windowsum = window.iter().sum();
            if windowsum > last {
                increases += 1;
            }
            last = windowsum;
            rotidx = (rotidx + 1) % 3;
        }
        println!("{}", increases - 2 /* first two values were from incomplete windows */ );
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
