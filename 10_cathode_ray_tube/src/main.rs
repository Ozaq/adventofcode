use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Op {
    Noop,
    Addx(i32),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some(("addx", val)) => Ok(Op::Addx(val.parse().unwrap())),
            None => Ok(Op::Noop),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
struct CPU {
    cycle: u32,
    reg_x: i32,
    program: Vec<Op>,
    instr_ptr: usize,
    wait_cycle: bool,
    signal_strength: i32,
}

impl CPU {
    fn new(program_text: &Vec<String>) -> Self {
        let program = program_text
            .iter()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();
        Self {
            cycle: 1,
            reg_x: 1,
            program,
            instr_ptr: 0,
            wait_cycle: false,
            signal_strength: 1,
        }
    }

    fn run_cycle(&mut self) {
        assert!(self.instr_ptr < self.program.len());
        self.signal_strength = self.cycle as i32 * self.reg_x;
        match self.program[self.instr_ptr] {
            Op::Noop => {
                self.instr_ptr += 1;
            }
            Op::Addx(x) => {
                self.wait_cycle = !self.wait_cycle;
                if !self.wait_cycle {
                    self.instr_ptr += 1;
                    self.reg_x += x;
                }
            }
        }
        self.cycle += 1;
    }
}

struct CRT {
    pixels: Vec<Vec<char>>,
    cpu: CPU,
}

impl CRT {
    fn new(cpu: CPU) -> Self {
        Self {
            pixels: vec![vec!['.'; 40]; 6],
            cpu,
        }
    }

    fn display(&self) {
        for line in &self.pixels {
            println!("{}", line.iter().collect::<String>());
        }
    }

    fn update_screen(&mut self) {
        for row in &mut self.pixels.iter_mut() {
            for (col_idx, col) in &mut row.iter_mut().enumerate() {
                if self.cpu.reg_x.abs_diff(col_idx as i32) <= 1 {
                    *col = '#';
                }
                self.cpu.run_cycle();
            }
        }
    }
}

fn main() {
    let filename = env::args().nth(1).expect("Missing filename argument");
    let file = File::open(filename).expect("File not found");
    let lines = BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>();
    let probe_cycles = vec![20u32, 60, 100, 140, 180, 220];
    let mut cpu1 = CPU::new(&lines);
    let cpu2 = cpu1.clone();
    let mut res = 0;
    while cpu1.cycle <= *probe_cycles.last().unwrap() {
        if probe_cycles.contains(&cpu1.cycle) {
            cpu1.run_cycle();
            res += cpu1.signal_strength;
        } else {
            cpu1.run_cycle();
        }
    }
    println!("First Solution {res}");
    let mut crt = CRT::new(cpu2);
    crt.update_screen();
    crt.display();
}
