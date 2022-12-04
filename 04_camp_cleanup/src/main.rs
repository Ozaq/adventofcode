use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Range {
    from: i32,
    to: i32,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.from <= other.from && self.to >= other.to
    }
    fn overlap(&self, other: &Self) -> bool {
        (self.from >= other.from && self.from <= other.to)
            || (self.to >= other.from && self.to <= other.to)
            || (other.from >= self.from && other.from <= self.to)
            || (other.to >= self.from && other.to <= self.to)
    }
}

fn pt1(input: &Vec<String>) {
    let count = input
        .iter()
        .filter(|x| {
            let values = x
                .split(",")
                .flat_map(|s| s.split("-"))
                .map(|x| x.parse::<i32>())
                .flatten()
                .collect::<Vec<i32>>();
            let r1 = Range {
                from: values[0],
                to: values[1],
            };
            let r2 = Range {
                from: values[2],
                to: values[3],
            };
            r1.contains(&r2) || r2.contains(&r1)
        })
        .count();
    println!("First solution: {}", count);
}

fn pt2(input: &Vec<String>) {
    let count = input
        .iter()
        .filter(|x| {
            let values = x
                .split(",")
                .flat_map(|s| s.split("-"))
                .map(|x| x.parse::<i32>())
                .flatten()
                .collect::<Vec<i32>>();
            let r1 = Range {
                from: values[0],
                to: values[1],
            };
            let r2 = Range {
                from: values[2],
                to: values[3],
            };
            r1.overlap(&r2)
        })
        .count();
    println!("Second solution: {}", count);
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
