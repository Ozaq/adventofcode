use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
#[derive(Debug)]
struct Forrest {
    heights: Vec<Vec<i32>>,
    num_columns: usize,
    num_rows: usize,
}

impl Forrest {
    fn new(lines: &Vec<String>) -> Self {
        let num_columns = lines[0].len();
        let num_rows = lines.len();
        let mut heights = vec![vec![0i32; num_rows]; num_columns];
        lines.iter().enumerate().for_each(|(row_idx, columns)| {
            columns.chars().enumerate().for_each(|(column_idx, c)| {
                heights[row_idx][column_idx] = c.to_digit(10).unwrap() as i32
            })
        });
        Self {
            heights,
            num_columns,
            num_rows,
        }
    }
    fn count_visible_trees(&self) -> usize {
        let mut visibility = vec![vec![false; self.num_rows]; self.num_columns];
        for row_idx in 0..self.num_rows {
            let mut max = i32::MIN;
            for col_idx in 0..self.num_columns {
                let h = self.heights[col_idx][row_idx];
                if h > max {
                    visibility[col_idx][row_idx] = true;
                    max = h;
                }
            }
        }
        for row_idx in 0..self.num_rows {
            let mut max = i32::MIN;
            for col_idx in (0..self.num_columns).rev() {
                let h = self.heights[col_idx][row_idx];
                if h > max {
                    visibility[col_idx][row_idx] = true;
                    max = h;
                }
            }
        }
        for col_idx in 0..self.num_columns {
            let mut max = i32::MIN;
            for row_idx in 0..self.num_rows {
                let h = self.heights[col_idx][row_idx];
                if h > max {
                    visibility[col_idx][row_idx] = true;
                    max = h;
                }
            }
        }
        for col_idx in 0..self.num_columns {
            let mut max = i32::MIN;
            for row_idx in (0..self.num_rows).rev() {
                let h = self.heights[col_idx][row_idx];
                if h > max {
                    visibility[col_idx][row_idx] = true;
                    max = h;
                }
            }
        }
        visibility.iter().flatten().filter(|x| **x).count()
    }

    fn max_scenic_score(&self) -> i32 {
        let mut max_score = i32::MIN;
        for col_idx in 1..self.num_columns - 1 {
            for row_idx in 1..self.num_rows - 1 {
                let tree_house_height = self.heights[col_idx][row_idx];
                println!("{:?}", tree_house_height);
                let mut up_score = 0;
                for row_idx_view in (0..row_idx).rev() {
                    up_score += 1;
                    if self.heights[col_idx][row_idx_view] >= tree_house_height {
                        break;
                    }
                }
                let mut down_score = 0;
                for row_idx_view in (row_idx + 1)..self.num_rows {
                    down_score += 1;
                    if self.heights[col_idx][row_idx_view] >= tree_house_height {
                        break;
                    }
                }
                let mut left_score = 0;
                for col_idx_view in (0..col_idx).rev() {
                    left_score += 1;
                    if self.heights[col_idx_view][row_idx] >= tree_house_height {
                        break;
                    }
                }
                let mut right_score = 0;
                for col_idx_view in (col_idx + 1)..self.num_columns {
                    right_score += 1;
                    if self.heights[col_idx_view][row_idx] >= tree_house_height {
                        break;
                    }
                }
                println!("{left_score}, {right_score}, {up_score}, {down_score}");
                max_score = max_score.max(left_score * right_score * up_score * down_score);
            }
        }
        max_score
    }
}

fn main() {
    let filename = env::args().nth(1).expect("Missing filename argument");
    let file = File::open(filename).expect("File not found");
    let lines = BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>();
    let f = Forrest::new(&lines);
    let num_visible_trees = f.count_visible_trees();
    println!("First solution {num_visible_trees}");
    println!("Second solution {}", f.max_scenic_score());
}
