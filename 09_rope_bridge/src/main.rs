use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<Direction> for Pos {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => Pos::new(0, 1),
            Direction::Right => Pos::new(1, 0),
            Direction::Down => Pos::new(0, -1),
            Direction::Left => Pos::new(-1, 0),
        }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

fn move_tail(tail: &Pos, head: &Pos) -> Pos {
    let dist = *head - *tail;
    if dist.x.abs() < 2 && dist.y.abs() < 2 {
        Pos::default()
    } else if dist.x == 0 {
        Pos::new(0, dist.y.signum())
    } else if dist.y == 0 {
        Pos::new(dist.x.signum(), 0)
    } else {
        Pos::new(dist.x.signum(), dist.y.signum())
    }
}

struct Move {
    count: u32,
    dir: Direction,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, count) = s.split_once(' ').unwrap();
        let count = count.parse::<u32>().unwrap();
        let dir = dir.parse().unwrap();
        Ok(Self { count, dir })
    }
}

fn get_two_mut<T>(data: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    assert!(a != b);
    let ptr: *mut [T] = data;
    unsafe { (&mut (*ptr)[a], &mut (*ptr)[b]) }
}

fn simulate_rope(len: usize, lines: &Vec<String>) -> HashSet<Pos> {
    let mut rope = vec![Pos::default(); len];
    let mut unique_position_counter = HashSet::new();
    unique_position_counter.insert(*rope.last().unwrap());
    for line in lines {
        let mov = line.parse::<Move>().unwrap();
        for _ in 0..mov.count {
            rope[0] += Pos::from(mov.dir);
            for idx in 1..rope.len() {
                let (head, tail) = get_two_mut(&mut rope, idx - 1, idx);
                *tail += move_tail(&tail, &head);
            }
            unique_position_counter.insert(*rope.last().unwrap());
        }
    }
    unique_position_counter
}

fn main() {
    let filename = env::args().nth(1).expect("Missing filename argument");
    let file = File::open(filename).expect("File not found");
    let lines = BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>();
    println!("First solution {}", simulate_rope(2, &lines).len());
    println!("Second solution {}", simulate_rope(10, &lines).len());
}
