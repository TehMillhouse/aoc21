use std::env;
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut pos: (i32, i32) = (0, 0);
    for cmd in parse(Path::new(filename)) {
        dbg!(cmd);
    }

}

fn parse(filename: &Path) -> impl std::iter::Iterator<Item = Cmd> {
    read_lines(filename).unwrap().map(|line| {
        let line = line.unwrap();
        let items = line.split(' ').collect::<Vec<_>>();
        let dir = match &items[0] {
            &"forward" => Dir::Forward,
            &"down" => Dir::Down,
            &"up" => Dir::Up,
            &_ => panic!(),
        };
        Cmd {
            dir: dir,
            dist: items[1].parse::<i32>().unwrap(),
        }
    })

}

fn read_lines(filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
