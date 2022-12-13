use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone)]
struct Map {
    width: u32,
    height: u32,
    height_data: Vec<Vec<u8>>,
    distance_data: Vec<Vec<u32>>,
    start: (i32, i32),
    goal: (i32, i32),
}

impl Map {
    fn new(lines: &Vec<String>) -> Self {
        let height = lines.len() as u32;
        let width = lines[0].len() as u32;
        let mut height_data = Vec::new();
        height_data.reserve(height as usize);
        for line in lines {
            height_data.push(Vec::from_iter(line.chars().map(|c| match c {
                ('a'..='z') => c as u8 - 'a' as u8,
                'S' => 0,
                'E' => 'z' as u8 - 'a' as u8,
                _ => panic!(),
            })));
        }
        let mut start = (0, 0);
        let mut goal = (0, 0);
        for (flat_idx, c) in lines.iter().map(|x| x.chars()).flatten().enumerate() {
            match c {
                'S' => {
                    start = (
                        flat_idx as i32 % width as i32,
                        flat_idx as i32 / width as i32,
                    );
                }
                'E' => {
                    goal = (
                        flat_idx as i32 % width as i32,
                        flat_idx as i32 / width as i32,
                    );
                }
                _ => {}
            }
        }
        let distance_data = vec![vec![u32::MAX; height_data[0].len()]; height_data.len()];
        Self {
            width,
            height,
            height_data,
            distance_data,
            start,
            goal,
        }
    }

    fn is_valid_neighbor(&self, pos: (i32, i32), source_height: u8) -> bool {
        (0..self.width).contains(&(pos.0 as u32))
            && (0..self.height).contains(&(pos.1 as u32))
            && self.h(pos) as i32 - source_height as i32 <= 1
    }

    fn accessible_neighbors(&self, pos: (i32, i32)) -> [Option<(i32, i32)>; 4] {
        let height = self.h(pos);
        let candidates = [
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ];
        [
            if self.is_valid_neighbor(candidates[0], height) {
                Some(candidates[0])
            } else {
                None
            },
            if self.is_valid_neighbor(candidates[1], height) {
                Some(candidates[1])
            } else {
                None
            },
            if self.is_valid_neighbor(candidates[2], height) {
                Some(candidates[2])
            } else {
                None
            },
            if self.is_valid_neighbor(candidates[3], height) {
                Some(candidates[3])
            } else {
                None
            },
        ]
    }

    fn dist(&self, (x, y): (i32, i32)) -> u32 {
        self.distance_data[y as usize][x as usize]
    }

    fn dist_mut(&mut self, (x, y): (i32, i32)) -> &mut u32 {
        &mut self.distance_data[y as usize][x as usize]
    }

    fn h(&self, (x, y): (i32, i32)) -> u8 {
        self.height_data[y as usize][x as usize]
    }

    fn lowest_cost_from_start(mut self) -> u32 {
        *self.dist_mut(self.start) = 0;
        let mut to_visit = VecDeque::new();
        to_visit.push_back(self.start);

        while let Some(pos) = to_visit.pop_front() {
            let distance = self.dist(pos) + 1;
            for n in self.accessible_neighbors(pos) {
                if let Some(n) = n {
                    if distance < self.dist(n) {
                        *self.dist_mut(n) = distance;
                        to_visit.push_back(n);
                    }
                }
            }
        }
        self.dist(self.goal)
    }

    fn all_zero_heights(&self) -> Vec<(i32, i32)> {
        self.height_data
            .iter()
            .flatten()
            .enumerate()
            .filter_map(|(flat_idx, val)| {
                if *val == 0 {
                    Some((
                        flat_idx as i32 % self.width as i32,
                        flat_idx as i32 / self.width as i32,
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    fn scenic_route_cost(mut self) -> u32 {
        let mut to_visit = VecDeque::new();
        for pos in self.all_zero_heights() {
            *self.dist_mut(pos) = 0;
            to_visit.push_back(pos);
        }

        while let Some(pos) = to_visit.pop_front() {
            let distance = self.dist(pos) + 1;
            for n in self.accessible_neighbors(pos) {
                if let Some(n) = n {
                    if distance < self.dist(n) {
                        *self.dist_mut(n) = distance;
                        to_visit.push_back(n);
                    }
                }
            }
        }
        self.dist(self.goal)
    }
}

fn main() {
    let filename = env::args().nth(1).expect("Missing filename argument");
    let file = File::open(filename).expect("File not found");
    let lines = BufReader::new(file)
        .lines()
        .flatten()
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();
    let map = Map::new(&lines);
    let cost = map.clone().lowest_cost_from_start();
    println!("Distance {cost}");
    let cost = map.scenic_route_cost();
    println!("Scenic route {cost}");
}
