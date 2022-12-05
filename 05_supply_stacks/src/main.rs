use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use regex::Regex;

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

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: &Vec<String>) -> (Vec<VecDeque<char>>, Vec<Move>) {
    let mut stacks = vec![];
    let mut moves = vec![];
    let mut parse_stack = true;
    let mut st = VecDeque::<Vec<(usize, char)>>::new();
    let re_move = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for line in input {
        if parse_stack {
            if line.is_empty() {
                parse_stack = false;
            } else {
                let stack_info = line
                    .chars()
                    .enumerate()
                    .filter(|x| ('A'..='Z').contains(&x.1))
                    .collect::<Vec<(usize, char)>>();
                if !stack_info.is_empty() {
                    st.push_front(stack_info);
                }
            }
        } else {
            let captures = re_move.captures(line).unwrap();
            moves.push(Move {
                count: captures[1].parse::<usize>().unwrap(),
                from: captures[2].parse::<usize>().unwrap(),
                to: captures[3].parse::<usize>().unwrap(),
            });
        }
    }
    let num_stacks = st.iter().fold(0, |acc, x| acc.max(x.len()));
    stacks.resize(num_stacks, VecDeque::<char>::new());
    for stack_info in &st {
        stack_info
            .iter()
            .for_each(|x| stacks[(x.0 - 1) / 4].push_front(x.1));
    }
    (stacks, moves)
}

fn pt1(input: &Vec<String>) {
    let (mut stacks, moves) = parse_input(input);
    moves.iter().for_each(|m| {
        for _ in 0..m.count {
            let item = stacks[m.from - 1].pop_front().unwrap();
            stacks[m.to - 1].push_front(item);
        }
    });
    print!("First solution: ");
    stacks
        .iter()
        .filter(|x| x.front().is_some())
        .for_each(|stack| print!("{}", stack.front().unwrap()));
    print!("\n");
}

fn get_two_mut<T>(data: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    assert!(a != b);
    let ptr: *mut [T] = data;
    unsafe { (&mut (*ptr)[a], &mut (*ptr)[b]) }
}

fn pt2(input: &Vec<String>) {
    let (mut stacks, moves) = parse_input(input);
    moves.iter().for_each(|m| {
        let (from, to) = get_two_mut(&mut stacks, m.from - 1, m.to - 1);
        let new_from = from.split_off(m.count);
        while let Some(x) = from.pop_back() {
            to.push_front(x);
        }
        *from = new_from;
    });
    print!("Second solution: ");
    stacks
        .iter()
        .filter(|x| x.front().is_some())
        .for_each(|stack| print!("{}", stack.front().unwrap()));
    print!("\n");
}
