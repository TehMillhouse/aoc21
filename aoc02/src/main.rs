use std::env;
use std::str::FromStr;
use std::string::ParseError;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Dir {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
struct Cmd {
    dir: Dir,
    dist: i32,
}

impl FromStr for Cmd {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let items = line.split(' ').collect::<Vec<_>>();
        let dir = match &items[0] {
            &"forward" => Dir::Forward,
            &"down" => Dir::Down,
            &"up" => Dir::Up,
            &_ => panic!(),
        };
        Ok(Cmd {
            dir: dir,
            dist: items[1].parse::<i32>().unwrap(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut pos: (i32, i32) = (0, 0);
    let mut aim: i32 = 0;
    for cmd in parse(Path::new(filename)) {
        match &cmd.dir {
            Dir::Down => aim += cmd.dist,
            Dir::Up => aim -= cmd.dist,
            Dir::Forward => 
                pos = (pos.0 + cmd.dist, pos.1 + aim * cmd.dist),
        };
    };
    println!("({}, {})", pos.0, pos.1);
    println!("{}", pos.0 * pos.1);

}

fn parse(filename: &Path) -> impl std::iter::Iterator<Item = Cmd> {
    read_lines(filename).unwrap().map(|line| {
        let line = line.unwrap();
        str::parse::<Cmd>(&line).unwrap()
    })

}

fn read_lines(filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
