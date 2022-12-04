use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn score(val: char) -> i32 {
    match val {
        'a'..='z' => val as i32 - 96,
        'A'..='Z' => val as i32 - 64 + 26,
        _ => panic!("WTF"),
    }
}

fn pt1(input: &Vec<String>) {
    let score = input.iter().fold(0i32, |sum, line| {
        let half_idx = line.len() / 2;
        let a = line.chars().take(half_idx).collect::<HashSet<char>>();
        let b = line.chars().skip(half_idx).collect::<HashSet<char>>();
        sum + a
            .intersection(&b)
            .fold(0i32, |acc, item| acc + score(*item))
    });
    println!("Solution of first part: {}", score);
}
fn pt2(input: &Vec<String>) {
    let score = input
        .iter()
        .step_by(3)
        .zip(input.iter().skip(1).step_by(3))
        .zip(input.iter().skip(2).step_by(3))
        .fold(0i32, |acc, item| {
            let ((first_str, second_str), third_str) = item;
            let first_set = first_str.chars().collect::<HashSet<char>>();
            let second_set = second_str.chars().collect::<HashSet<char>>();
            let third_set = third_str.chars().collect::<HashSet<char>>();
            acc + first_set
                .intersection(&second_set)
                .cloned()
                .collect::<HashSet<_>>()
                .intersection(&third_set)
                .fold(0i32, |acc_inner, item| acc_inner + score(*item))
        });
    println!("Solution of second part: {}", score);
}

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
