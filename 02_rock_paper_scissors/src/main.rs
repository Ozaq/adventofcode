use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Copy)]
enum Gesture {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Gesture {
    fn from_char(input: char) -> Result<Gesture, ()> {
        match input {
            'A' => Ok(Gesture::Rock),
            'B' => Ok(Gesture::Paper),
            'C' => Ok(Gesture::Scissors),
            _ => Err(()),
        }
    }

    fn draw(&self) -> Gesture {
        *self
    }

    fn win(&self) -> Gesture {
        match self {
            Gesture::Rock => Gesture::Paper,
            Gesture::Paper => Gesture::Scissors,
            Gesture::Scissors => Gesture::Rock,
        }
    }

    fn lose(&self) -> Gesture {
        match self {
            Gesture::Rock => Gesture::Scissors,
            Gesture::Paper => Gesture::Rock,
            Gesture::Scissors => Gesture::Paper,
        }
    }
}

struct Play {
    opponent: Gesture,
    you: char,
}

fn score_play(you: Gesture, opponent: Gesture) -> i32 {
    match you {
        Gesture::Rock => match opponent {
            Gesture::Rock => 3,
            Gesture::Paper => 0,
            Gesture::Scissors => 6,
        },
        Gesture::Paper => match opponent {
            Gesture::Rock => 6,
            Gesture::Paper => 3,
            Gesture::Scissors => 0,
        },
        Gesture::Scissors => match opponent {
            Gesture::Rock => 0,
            Gesture::Paper => 6,
            Gesture::Scissors => 3,
        },
    }
}

fn score(plays: &Vec<Play>) -> i32 {
    plays.into_iter().fold(0i32, |acc, item| {
        let your_gesture = match item.you {
            'X' => item.opponent.lose(),
            'Y' => item.opponent.draw(),
            'Z' => item.opponent.win(),
            _ => panic!("Should not happen :D"),
        };
        acc + score_play(your_gesture, item.opponent) + your_gesture as i32
    })
}

fn main() {
    let filename = env::args().nth(1).expect("Missing filename argument");
    let file = File::open(filename).expect("File not found");
    let lines = BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>();
    let plays: Vec<Play> = lines
        .iter()
        .map(|line| Play {
            opponent: Gesture::from_char(line.chars().nth(0).unwrap()).unwrap(),
            you: line.chars().nth(2).unwrap(),
        })
        .collect();
    let score = score(&plays);
    println!("Score: {}", score);
}
