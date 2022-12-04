use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::string::String;

fn main() {
    let filename = env::args().nth(1).expect("Missing filename argument");
    let file = File::open(filename).expect("File not found");
    let lines = BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>();
    let mut calories = Vec::<u64>::new();
    let mut current_calories = 0u64;
    for line in &lines {
        if line.is_empty() {
            calories.push(current_calories);
            current_calories = 0;
            continue;
        }
        current_calories += line.parse::<u64>().unwrap();
    }
    calories.push(current_calories);
    calories.sort_by(|a, b| b.cmp(a));
    let top_three = calories[0] + calories[1] + calories[2];
    println!("max calories {}", calories[0]);
    println!("top 3 calories {}", top_three);
}
