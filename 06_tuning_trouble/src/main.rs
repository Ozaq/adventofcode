use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let filename = env::args().nth(1).expect("Missing filename argument");
    let file = File::open(filename).expect("File not found");
    let lines = BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>();
    pt1(&lines);
    pt2(&lines);
}

fn pt1(input: &Vec<String>) {
    let mut paket = VecDeque::<char>::new();
    for (idx, c) in input.first().unwrap().chars().enumerate() {
        paket.push_back(c);
        if paket.len() == 4 {
            if paket.iter().collect::<HashSet<_>>().len() == 4 {
                paket.iter().for_each(|x| print!("{}", x));
                println!("\nCount: {}", idx + 1);
                break;
            }
            paket.pop_front();
        }
    }
}

fn pt2(input: &Vec<String>) {
    let mut paket = VecDeque::<char>::new();
    for (idx, c) in input.first().unwrap().chars().enumerate() {
        paket.push_back(c);
        if paket.len() == 14 {
            if paket.iter().collect::<HashSet<_>>().len() == 14 {
                paket.iter().for_each(|x| print!("{}", x));
                println!("\nCount: {}", idx + 1);
                break;
            }
            paket.pop_front();
        }
    }
}
